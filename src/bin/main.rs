#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate PhotoManagerLib;

extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;

use PhotoManagerLib::create_db_pool;
use PhotoManagerLib::controllers;


#[get("/static/<file..>")]
fn static_content(file: std::path::PathBuf) -> Option<rocket::response::NamedFile> {
    rocket::response::NamedFile::open(std::path::Path::new("static/").join(file)).ok()
}

#[get("/favicon.ico")]
fn favicon() -> rocket::response::Redirect {
  rocket::response::Redirect::permanent("/static/favicon.ico")
}

fn main() {
  rocket::ignite()
    .manage(create_db_pool()) // Register connection pool
    .mount("/", routes![controllers::photo::index, controllers::photo::new, controllers::photo::show, controllers::photo::delete,
                                    static_content, favicon])
    .attach(rocket_contrib::Template::fairing())
    .launch();
}