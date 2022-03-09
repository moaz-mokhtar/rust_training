// use log::info;

pub fn initiate_logging() {
    std::env::set_var("RUST_LOG", "debug, actix_web=debug");

    let env = dotenv::from_filename(".env").expect("'.env' not found.");
    dbg!(env);

    if std::env::var("PWD").is_err() {
        std::env::set_var("PWD", env!("CARGO_MANIFEST_DIR"));
        let pwd = std::env::var("PWD").unwrap();
        dbg!(pwd);
    }

    env_logger::init();
}
