use crate::auth::{
    decode_access_token, decode_refresh_token, generate_access_token, generate_refresh_token,
    get_auth_from_header,
};
use crate::entity::general::{MessageResponse, TokenResponse};
use crate::entity::user::{
    ForgotRequest, Reset, ResetRequest, UserLoginRequest, UserRegisterationRequest, UserToken,
};
use crate::{db::DbPool, entity::user::User};
use actix_identity::Identity;
use actix_session::{Session, SessionExt};
use actix_web::HttpRequest;
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    Error, HttpResponse, Responder,
};
use chrono::Duration;
use lettre::{ClientSecurity, Message, SmtpTransport, Transport};
// use cookie::{Cookie, CookieJar};
use log::{debug, info};
use rand::distributions::{Alphanumeric, DistString};

pub fn routes_config(config: &mut ServiceConfig) {
    config
        .service(health)
        .service(register)
        .service(login)
        .service(get_user)
        .service(refresh)
        .service(logout)
        .service(forgot)
        .service(reset);
}

// TODO: API list:
/*
    register
    login
    user
    refresh
    logout
    forgot
    reset
    two-factor
    google-auth
*/

#[get("/")]
/// Route to test API functionality without any communcation with DB.
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[post("/register")]
pub async fn register(
    data: web::Json<UserRegisterationRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", data.0.clone());

    if data.0.password != data.0.password_confirm {
        return Ok(HttpResponse::BadRequest().json("Kindly confirm the same password."));
    }

    match User::insert(data.0, &pool).await {
        Ok(feedback) => Ok(HttpResponse::Ok().json(feedback)),
        Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
    }
}

#[post("/login")]
pub async fn login(
    data: web::Json<UserLoginRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    match User::authenticate_by_email(data.0.email, data.0.password, &pool).await {
        Ok(user) => {
            let user_id = user.id;
            let access_token = generate_access_token(user_id);
            let refresh_token = generate_refresh_token(user_id);

            info!("access_token {}", access_token);
            info!("refresh_token {}", refresh_token);

            let now = chrono::Utc::now();

            let user_token = UserToken {
                user_id: user_id,
                token: refresh_token.clone(),
                created_at: now.naive_utc(),
                expires_at: now.naive_utc() + Duration::days(7),
            };

            match UserToken::insert(user_token, &pool).await {
                Ok(_) => {
                    let cookie =
                        actix_web::cookie::Cookie::build("refresh_token", refresh_token.clone())
                            .http_only(true)
                            .finish();

                    let response = TokenResponse {
                        token: access_token,
                    };

                    Ok(HttpResponse::Ok().cookie(cookie).json(response))
                }
                Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[get("/user")]
pub async fn get_user(
    request: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    match get_auth_from_header(request) {
        Ok(auth_token) => {
            dbg!(auth_token.clone());

            match decode_access_token(auth_token) {
                Ok(user_id) => {
                    let id = uuid::Uuid::parse_str(&user_id).unwrap();
                    match User::find_by_id(id, &pool).await {
                        Ok(user) => Ok(HttpResponse::Ok().json(user)),
                        Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
                    }
                }
                Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[get("/refresh")]
pub async fn refresh(request: HttpRequest, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    match request.cookie("refresh_token") {
        Some(cookie) => {
            let refresh_token = cookie.value();
            dbg!(refresh_token.clone());
            match decode_refresh_token(refresh_token.to_string()) {
                Ok(user_id) => {
                    let id = uuid::Uuid::parse_str(&user_id).unwrap();
                    match User::find_by_id(id, &pool).await {
                        Ok(user) => {
                            // if token for this user not exist, return unauthenticated error
                            match UserToken::find_by_token(refresh_token.to_string(), id, &pool)
                                .await
                            {
                                Ok(user_token) => {
                                    let now = chrono::Utc::now().naive_utc();
                                    if user_token.expires_at < now {
                                        return Ok(
                                            HttpResponse::Unauthorized().json("Token expired")
                                        );
                                    }

                                    let access_token = generate_access_token(user.id);
                                    let response = TokenResponse {
                                        token: access_token,
                                    };

                                    Ok(HttpResponse::Ok().json(response))
                                }
                                Err(_) => Ok(HttpResponse::Unauthorized().json("Token not found")),
                            }
                        }
                        Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
                    }
                }
                Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
            }
        }
        None => Ok(HttpResponse::Unauthorized().json("No `refresh_token` found")),
    }
}

#[get("/logout")]
pub async fn logout(request: HttpRequest, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let response = MessageResponse {
        message: format!("success"),
    };

    let mut http_response = HttpResponse::Ok().json(response);
    // let cookie_to_remove = request.cookie("refresh_token").unwrap();
    match request.cookie("refresh_token") {
        Some(cookie) => {
            let refresh_token = cookie.value();

            info!("/logout -> refresh_token: {:?}", refresh_token);

            match UserToken::delete(refresh_token.to_string(), &pool).await {
                Ok(_) => {
                    info!("/logout -> cookie_to_remove: {:?}", cookie.clone());
                    http_response.add_removal_cookie(&cookie).unwrap();
                    info!("/logout -> response: {:?}", http_response);
                    Ok(http_response)
                }
                Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
            }
        }
        None => Ok(HttpResponse::Unauthorized().json("No `refresh_token` found")),
    }
}

#[post("/forgot")]
pub async fn forgot(
    data: web::Json<ForgotRequest>,
    request: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let email = data.0.email;
    let token = (Alphanumeric.sample_string(&mut rand::thread_rng(), 10)).to_lowercase();
    info!("/forgot -> email: {}", &email);
    info!("/forgot -> rand token: {}", &token);

    match Reset::insert(email.clone(), token.clone(), &pool).await {
        Ok(_) => {
            let frontend_url =
                std::env::var("FRONTEND_URL").expect("Missed 'FRONTEND_URL' environment variable");
            dbg!(&frontend_url);

            let reset_url = format!("{frontend_url}/reset/{token}");
            dbg!(&reset_url);

            let email_body = format!("Click <a href={reset_url}> here</a> to reset password");
            let email = lettre_email::EmailBuilder::new()
                .to("hello@example.com")
                .from("no-reply@site.com")
                .subject("Reset your password")
                .html(email_body)
                .build()
                .unwrap();

            info!("/forgot -> email: {:?}", &email);

            let mut mailer = lettre::SmtpClient::new("localhost:1025", ClientSecurity::None)
                .unwrap()
                .transport();

            let result = mailer.send(email.into());
            info!("/forgot -> mailer.send: {:?}", &result);

            let response = MessageResponse {
                message: format!("success"),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

#[post("/reset")]
pub async fn reset(
    data: web::Json<ResetRequest>,
    request: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    // check that password is the same with password_confirmation
    if data.0.password != data.0.password_confirm {
        return Ok(HttpResponse::BadRequest().json("Passwords do not match!"));
    }

    info!("/reset -> incoming_data: {:?}", &data);
    let incoming_token = data.0.token;
    let incoming_password = data.0.password;

    // get Reset object related to incoming_token from db
    match Reset::find_by_token(incoming_token.clone(), &pool).await {
        Ok(reset) => {
            info!("/reset -> reset: {:?}", &reset);

            // if incoming_token not same with token from db, return "Invalid link!"
            if reset.token != incoming_token {
                return Ok(HttpResponse::BadRequest().json("Invalid link!"));
            }
            let email = reset.email.clone();

            // get User object related to email of Reset object.
            match User::find_by_email(email, &pool).await {
                Ok(user) => {
                    info!("/reset -> user: {:?}", &user);

                    // update User object password
                    // Return message:success
                    match User::update_password(user.id, incoming_password, &pool).await {
                        Ok(_) => {
                            let response = MessageResponse {
                                message: format!("success"),
                            };
                            Ok(HttpResponse::Ok().json(response))
                        }
                        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
                    }
                }
                // if User object not found, return "User not found!"
                Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
    }
}
