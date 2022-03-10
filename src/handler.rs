use crate::entity::general::Response;
use crate::entity::user::{UserLogin, UserRegisteration};
use crate::{db::DbPool, entity::user::User};

use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    Error, HttpResponse, Responder,
};

pub fn routes_config(config: &mut ServiceConfig) {
    config
    .service(health)
    .service(get_users)
    .service(register)
    .service(login)
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

    match User::authenticate(user_login.0.username, user_login.0.password, &pool).await {
        Ok(is_authenticated) => {
            if is_authenticated {
                let response = Response {
                    message: format!("User authenticated"),
                    data: is_authenticated,
                };

                let serialized_response = serde_json::to_string(&response).unwrap();

                Ok(HttpResponse::Ok().body(serialized_response))
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
                serde_json::to_string(format!("BadRequest: {}", error_message).as_str()).unwrap();
            Ok(HttpResponse::BadRequest().body(body_message))
        }
    }
}

//=============
//=============

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let body_message;

    match User::get_all(&pool).await {
        Ok(users_data) => {
            let response = Response {
                message: format!("List of users"),
                data: users_data,
            };

            let serialized_response = serde_json::to_string(&response).unwrap();

            Ok(HttpResponse::Ok().body(serialized_response))
        }
        Err(e) => {
            let error_message = e.to_string();

            body_message =
                serde_json::to_string(format!("Internal Server Error: {}", error_message).as_str())
                    .unwrap();
            Ok(HttpResponse::InternalServerError().body(body_message))
        }
    }
}

// pub async fn add(
//     pool: web::Data<DbPool>,
//     new_request: web::Json<NewStudent>,
// ) -> Result<HttpResponse, Error> {
//     let connection = pool.get_ref();
//     let new_student = NewStudent {
//         name: new_request.name.clone(),
//         image_path: new_request.image_path.clone(),
//     };
//     println!("inside add");
//     //let new_student= new_student.as_ref();
//     let _students_data = Student::insert(new_student, connection)
//         .await
//         .map_err(|_| HttpResponse::InternalServerError())?;

//     println!("inside add2");
//     Ok(HttpResponse::Ok().body("success"))
// }

// pub async fn get_by_id(
//     pool: web::Data<DbPool>,
//     record_id: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {
//     let connection = pool.get_ref();
//     let record_id = record_id.into_inner();
//     let record_data = Student::get_by_id(connection, record_id)
//         .await
//         .map_err(|_| HttpResponse::InternalServerError().finish())?;

//     let body = serde_json::to_string(&record_data).unwrap(); //hb.render("student", &student_data).unwrap();

//     Ok(HttpResponse::Ok().body(body))
// }

// pub async fn archive_by_id(
//     pool: web::Data<DbPool>,
//     record_id: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {
//     let connection = pool.get_ref();
//     let record_id = record_id.into_inner();
//     let record_data = Student::archive_by_id(connection, record_id)
//         .await
//         .map_err(|_| HttpResponse::InternalServerError().finish())?;

//     let body = serde_json::to_string(&record_data).unwrap();
//     Ok(HttpResponse::Ok().body(body))
// }
