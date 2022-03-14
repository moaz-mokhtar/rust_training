#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

pub mod auth;
pub mod db;
pub mod engine;
pub mod entity;
pub mod handler;
pub mod schema;
pub mod utils;

type MyError = Box<dyn std::error::Error + Send + Sync>;
