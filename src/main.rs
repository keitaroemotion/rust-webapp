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
    if let Some(person) = person {
        if let Some(entry) = person.entry {
            format!("Hello, {} year old categoryd {}!", entry, person.category)
        } else {
            format!("Hello {}!", person.category)
        }
    } else {
        "We're gonna need a category, and only a category.".into()
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
