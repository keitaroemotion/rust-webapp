#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::request::{Form, LenientForm};

#[derive(FromForm)]
struct Blog {
    category: String,
    entry:    Option<u8>
}

#[get("/blog?<article..>")]
fn blog(article: Option<LenientForm<Blog>>) -> String {
    match article {
        Some(x) => if let Some(entry) = x.entry {
                       format!("category: {} entry: {}", x.category, entry)
                   } else {
                       format!("category: {} (articles enlisted here:)", x.category) 
                   }   
        None    => "wrong input".into()
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
                            blog,
                        ]
                    )
                    .launch();
}
