#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

fn main() {
    rocket::ignite().mount("/", routes![index, health]).launch();
}