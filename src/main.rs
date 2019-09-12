#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/blog/<category>/<id>")]
fn blog_category_id(category: String, id: String) -> String {
    format!("blog entry is: [{}] {}", category, id)
}

#[get("/blog/<category>")]
fn blog_category(category: String) -> String {
    format!("blog category is: [{}]", category)
}

#[get("/blog")]
fn blog() ->  &'static str {
    "blog" 
}

fn main() {
    rocket::ignite().mount(
                        "/",
                        routes![
                            index,
                            blog,
                            blog_category,
                            blog_category_id
                        ]
                    )
                    .launch();
}
