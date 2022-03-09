use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use log::info;
use rust_training::{db::DbClientConn, handler, utils};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    info!("Welcome by Moaz bin Mohamed Mokhtar");
    utils::initiate_logging();

    // info!("address: {}", address);
    // let server = Server::http(address.as_str()).expect("Failed to start server.");
    // info!("Tiny server started");

    let pool = DbClientConn::get_pool_connection();
    let data = Data::new(pool);

    let address = std::env::var("ADDRESS").expect("Missed 'ADDRESS' environment variable");
    // let address = "127.0.0.1:8080";

    info!("Server address: {address}");
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(handler::routes_config)
    })
    .bind(address)?
    .run()
    .await
}
