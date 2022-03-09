use std::sync::atomic::fence;

use crate::engine::*;
use crate::entity::general::Response;
use crate::entity::user::{UserLogin, UserRegisteration};
use crate::{
    db::DbPool,
    entity::user::{User, UserDTO},
};

use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    Error, HttpResponse, Responder,
};
// use crate::db::DbPool;
// use crate::entity::*;

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
    if new_user.0.password != new_user.0.confirm_password {
        let message = serde_json::to_string("Kindly confirm the same password.").unwrap();
        return Ok(HttpResponse::Ok().body(message));
    }

    let feedback = User::insert(new_user.0, &pool).await.unwrap();

    let message = serde_json::to_string(&feedback).unwrap();

    Ok(HttpResponse::Ok().body(message))
}

#[post("/login")]
pub async fn login(
    user_login: web::Json<UserLogin>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let mut is_auth = false;

    if let Ok(feedback) =
        User::authenticate(user_login.0.username, user_login.0.password, &pool).await
    {
        if feedback {
            is_auth = true;
        }
    }

    let message = serde_json::to_string(&is_auth).unwrap();

    Ok(HttpResponse::Ok().body(message))
}

//=============
//=============

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users_data = User::get_all(&pool).await.unwrap();

    let res = serde_json::to_string(&users_data).unwrap();

    Ok(HttpResponse::Ok().body(res))
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
