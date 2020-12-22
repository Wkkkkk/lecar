#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use crate::routes::{launch_server};

mod routes;

fn main() {
    launch_server();
}