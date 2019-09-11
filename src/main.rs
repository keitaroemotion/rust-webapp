#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/blog/<id>")]
fn blog(id: &RawStr) -> String {
    format!("blog entry is: {}", id.as_str())
}

fn main() {
    rocket::ignite().mount("/", routes![index, blog]).launch();
}
