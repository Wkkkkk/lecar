#![feature(binary_heap_retain)]
#![feature(associated_type_defaults)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rand::prelude::*;
use crate::routes::{launch_server};

mod cache;
mod controller;
mod enums;
mod routes;

#[cfg(test)]
mod tests;

fn main() {
    launch_server();
}
