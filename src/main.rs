#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::request::Form;

#[derive(FromForm)]
struct Person {
    category:  String,
    entry: Option<u8>
}

#[get("/blog?<person..>")]
fn blog(person: Option<Form<Person>>) -> String {
    match person {
      Some(x) => if let Some(entry) = x.entry {
                     format!("Hello, {} year old categoryd {}!", entry, x.category)
                 } else {
                     format!("Hello {}!", x.category) 
                 }   
      None    => "aaa".into()
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount(
                        "/",
                        routes![
                            index,
                            blog
                        ]
                    )
                    .launch();
}
