use diesel::{r2d2::ConnectionManager, PgConnection};
// use diesel::{MysqlConnection};
use dotenv::dotenv;
use log::info;
use r2d2::Pool;
use std::env;

// pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
// The Postgres-specific connection pool managing all database connections.
// pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbClientConn;
impl DbClientConn {
    pub fn get_pool_connection() -> DbPool {
        // it from the environment within this function
        dotenv().ok();
        let url = env::var("DATABASE_URL").expect("Couldn't found 'DATABASE_URL' inside .env file");
        info!("DATABASE_URL: {url}");
        // TODO let migr = ConnectionManager::<MysqlConnection>::new(url);
        let migr = ConnectionManager::<PgConnection>::new(url);
        r2d2::Pool::builder()
            .build(migr)
            .expect("could not build connection pool")
    }
}
