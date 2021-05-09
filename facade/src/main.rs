#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io::Read;

use rocket::response::content::Json;

#[get("/")]
fn index() -> Json<String> {
    let mut response = String::new();
    reqwest::blocking::get("localhost:30463/")
        .expect("Something went wrong")
        .read_to_string(&mut response)
        .expect("Got no response");
    Json(format!("{{\"response\":\"{}\"}}", response))
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

fn main() {
    rocket::ignite().mount("/", routes![index, health]).launch();
}