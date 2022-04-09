use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use log::info;
use rust_training::{db::DbClientConn, handler, utils};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    info!("Welcome to the Rust_Training");
    utils::initiate_logging();

    // info!("address: {}", address);
    // let server = Server::http(address.as_str()).expect("Failed to start server.");
    // info!("Tiny server started");

    let pool = DbClientConn::get_pool_connection();
    let data = Data::new(pool);

    let address = std::env::var("ADDRESS").expect("Missed 'ADDRESS' environment variable");

    let session_key = cookie::Key::generate();

    info!("Server address: {address}");
    HttpServer::new(move || {
        let identity_policy = CookieIdentityPolicy::new(&[0; 32])
            .name("auth-cookie")
            .secure(false);

        App::new()
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                session_key.clone(),
            ))
            .wrap(IdentityService::new(identity_policy))
            .app_data(data.clone())
            .service(web::scope("/api").configure(handler::routes_config))
    })
    .bind(address)?
    .run()
    .await
}
