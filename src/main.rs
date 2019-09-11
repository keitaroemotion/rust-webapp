#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/blog/<category>/<id>")]
fn blog(category: String, id: String) -> String {
    format!("blog entry is: [{}] {}", category, id)
}

fn main() {
    rocket::ignite().mount("/", routes![index, blog]).launch();
}
