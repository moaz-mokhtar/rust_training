#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

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

// type MyError = Box<dyn std::error::Error + Send + Sync>;

extern crate custom_error;
use custom_error::custom_error;

custom_error! { pub MyError
    General{desc:String}  = "Error: {desc}",
}
