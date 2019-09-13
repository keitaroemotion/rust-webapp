#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::request::LenientForm;
use rocket::Request;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
struct AssetsDir(String);

#[get("/<asset..>")]
fn assets(asset: PathBuf, assets_dir: State<AssetsDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&assets_dir.0).join(asset)).ok()
}

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
fn index() -> Template {
    let mut context  = HashMap::new();
    context.insert(0u32, 'x');
    Template::render("index", context)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

fn main() {
    rocket::ignite().mount(
                        "/",
                        routes![
                            index,
                            blog,
                        ]
                    )
                    .attach(Template::fairing())
                    .launch();
}
