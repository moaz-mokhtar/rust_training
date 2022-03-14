use crate::auth::{
    decode_access_token, decode_refresh_token, generate_access_token, generate_refresh_token,
    get_auth_from_header, parse_refresh_token_from_cookies,
};
use crate::entity::general::Response;
use crate::entity::user::{UserLogin, UserRegisteration};
use crate::{db::DbPool, entity::user::User};

use actix_web::HttpRequest;
use actix_web::{
    cookie::Cookie,
    get, post,
    web::{self, ServiceConfig},
    Error, HttpResponse, Responder,
};

pub fn routes_config(config: &mut ServiceConfig) {
    config
    .service(health)
    .service(get_users_list)
    .service(register)
    .service(login)
    .service(get_user)
    .service(refresh)
    // ===
    // .route("/student/{id}", web::get().to(get_by_id))
    // .route("/student/add", web::post().to(add))
    // .route("/student/archive/{id}", web::delete().to(archive_by_id))
    ;
}

// ===
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

// =====
// =====
// =====

#[get("/")]
/// Route to test API functionality without any communcation with DB.
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[post("/register")]
pub async fn register(
    new_user: web::Json<UserRegisteration>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let body_message;

    if new_user.0.password != new_user.0.confirm_password {
        let response = Response {
            message: format!("Kindly confirm the same password."),
            data: {},
        };

        let serialized_response = serde_json::to_string(&response).unwrap();

        return Ok(HttpResponse::BadRequest().body(serialized_response));

        // body_message = serde_json::to_string("Kindly confirm the same password.").unwrap();
        // return Ok(HttpResponse::BadRequest().body(body_message));
    }

    match User::insert(new_user.0, &pool).await {
        Ok(feedback) => {
            let response = Response {
                message: format!("New user added."),
                data: feedback,
            };

            let serialized_response = serde_json::to_string(&response).unwrap();

            // body_message = serde_json::to_string(&feedback).unwrap();
            Ok(HttpResponse::Ok().body(serialized_response))
        }
        Err(e) => {
            let error_message = e.to_string();

            body_message =
                serde_json::to_string(format!("BadRequest: {}", error_message).as_str()).unwrap();
            Ok(HttpResponse::BadRequest().body(body_message))
        }
    }
}

#[post("/login")]
pub async fn login(
    user_login: web::Json<UserLogin>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let body_message;
    match User::find_by_username(user_login.0.username.clone(), &pool).await {
        Ok(user) => {
            let user_id = user.id;

            match User::authenticate(user_login.0.username, user_login.0.password, &pool).await {
                Ok(is_authenticated) => {
                    if is_authenticated {
                        // ===
                        let _access_token = generate_access_token(user_id);
                        let refresh_token = generate_refresh_token(user_id);
                        let cookie = Cookie::build("refresh_token", refresh_token)
                            .http_only(true)
                            .finish();

                        let response = Response {
                            message: format!("User authenticated"),
                            data: is_authenticated,
                        };

                        let serialized_response = serde_json::to_string(&response).unwrap();

                        dbg!(_access_token);

                        Ok(HttpResponse::Ok().cookie(cookie).body(serialized_response))
                    } else {
                        let message =
                            format!("Credential not correct. Try again with correct credentials.");
                        body_message = serde_json::to_string(message.as_str()).unwrap();

                        Ok(HttpResponse::BadRequest().body(body_message))
                    }
                }
                Err(e) => {
                    let error_message = e.to_string();

                    body_message =
                        serde_json::to_string(format!("BadRequest: {}", error_message).as_str())
                            .unwrap();
                    Ok(HttpResponse::BadRequest().body(body_message))
                }
            }
        }
        Err(e) => {
            let error_message = e.to_string();

            body_message =
                serde_json::to_string(format!("BadRequest: {}", error_message).as_str()).unwrap();
            Ok(HttpResponse::BadRequest().body(body_message))
        }
    }
}

#[get("/user")]
pub async fn get_user(
    request: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let body_message;
    match get_auth_from_header(request) {
        Ok(auth_token) => {
            dbg!(auth_token.clone());

            match decode_access_token(auth_token) {
                Ok(user_id) => {
                    let id = uuid::Uuid::parse_str(&user_id).unwrap();
                    match User::find_by_id(id, &pool).await {
                        Ok(user) => {
                            let response = Response {
                                message: format!("User info"),
                                data: user,
                            };

                            let serialized_response = serde_json::to_string(&response).unwrap();

                            Ok(HttpResponse::Ok().body(serialized_response))
                        }
                        Err(e) => {
                            let error_message = e.to_string();

                            body_message = serde_json::to_string(
                                format!("BadRequest: {}", error_message).as_str(),
                            )
                            .unwrap();
                            Ok(HttpResponse::BadRequest().body(body_message))
                        }
                    }
                }
                Err(e) => {
                    let error_message = e.to_string();

                    body_message =
                        serde_json::to_string(format!("BadRequest: {}", error_message).as_str())
                            .unwrap();
                    Ok(HttpResponse::BadRequest().body(body_message))
                }
            }
        }
        Err(e) => {
            let error_message = e.to_string();

            body_message =
                serde_json::to_string(format!("BadRequest: {}", error_message).as_str()).unwrap();
            Ok(HttpResponse::BadRequest().body(body_message))
        }
    }
}

#[get("/refresh")]
pub async fn refresh(request: HttpRequest, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    // ===
    let body_message;

    match request.headers().get(actix_web::http::header::COOKIE) {
        Some(cookie_header) => {
            dbg!(cookie_header);

            match parse_refresh_token_from_cookies(cookie_header.to_str().unwrap()) {
                Some(refresh_token) => {
                    dbg!(refresh_token.clone());

                    match decode_refresh_token(refresh_token) {
                        Ok(user_id) => {
                            let id = uuid::Uuid::parse_str(&user_id).unwrap();
                            match User::find_by_id(id, &pool).await {
                                Ok(user) => {
                                    let access_token = generate_access_token(user.id.clone());

                                    let response = Response {
                                        message: format!("Token refreshed"),
                                        data: access_token,
                                    };

                                    let serialized_response =
                                        serde_json::to_string(&response).unwrap();

                                    Ok(HttpResponse::Ok().body(serialized_response))
                                }
                                Err(e) => {
                                    let error_message = e.to_string();

                                    body_message = serde_json::to_string(
                                        format!("BadRequest: {}", error_message).as_str(),
                                    )
                                    .unwrap();
                                    Ok(HttpResponse::BadRequest().body(body_message))
                                }
                            }
                        }
                        Err(e) => {
                            let error_message = e.to_string();

                            body_message = serde_json::to_string(
                                format!("BadRequest: {}", error_message).as_str(),
                            )
                            .unwrap();
                            Ok(HttpResponse::BadRequest().body(body_message))
                        }
                    }
                }
                None => {
                    let error_message = format!("No `refresh_token` found");
                    body_message =
                        serde_json::to_string(format!("BadRequest: {}", error_message).as_str())
                            .unwrap();
                    Ok(HttpResponse::BadRequest().body(body_message))
                }
            }
        }
        None => {
            let error_message = format!("No cookie header found");
            body_message =
                serde_json::to_string(format!("BadRequest: {}", error_message).as_str()).unwrap();
            Ok(HttpResponse::BadRequest().body(body_message))
        }
    }
}

//=============
//=============

#[get("/users")]
pub async fn get_users_list(
    request: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let body_message;
    match get_auth_from_header(request) {
        Ok(auth_token) => {
            dbg!(auth_token.clone());

            match decode_access_token(auth_token) {
                Ok(_) => {
                    // ===
                    match User::get_all(&pool).await {
                        Ok(user) => {
                            let response = Response {
                                message: format!("Users list"),
                                data: user,
                            };

                            let serialized_response = serde_json::to_string(&response).unwrap();

                            Ok(HttpResponse::Ok().body(serialized_response))
                        }
                        Err(e) => {
                            let error_message = e.to_string();

                            body_message = serde_json::to_string(
                                format!("BadRequest: {}", error_message).as_str(),
                            )
                            .unwrap();
                            Ok(HttpResponse::BadRequest().body(body_message))
                        }
                    }
                }
                Err(e) => {
                    let error_message = e.to_string();

                    body_message =
                        serde_json::to_string(format!("BadRequest: {}", error_message).as_str())
                            .unwrap();
                    Ok(HttpResponse::BadRequest().body(body_message))
                }
            }
        }
        Err(e) => {
            let error_message = e.to_string();

            body_message =
                serde_json::to_string(format!("BadRequest: {}", error_message).as_str()).unwrap();
            Ok(HttpResponse::BadRequest().body(body_message))
        }
    }
}
