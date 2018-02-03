#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate PhotoManagerLib;

extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use diesel::prelude::*;

use PhotoManagerLib::*;
use PhotoManagerLib::models::*;
use PhotoManagerLib::schema::*;

use rocket_contrib::Template;
use tera::Context;


#[get("/")]
fn index(connection: DbConn) -> Template {
  let mut context = Context::new();
  let photos = photo::table.load::<Photo>(&*connection).expect("Error loading photos");
  context.add("photos", &photos);

  Template::render("base", context)
}

#[post("/", data= "<photo_data>")]
fn new(connection: DbConn, photo_data: rocket::request::Form<NewPhoto>) -> rocket::response::Redirect {
  use PhotoManagerLib::schema::photo::dsl;
  use rocket::response::*;

  diesel::insert_into(dsl::photo)
  .values(photo_data.get())
  .execute(&*connection)
  .expect("Error inserting form data!");

  Redirect::to("/")
}

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
    .mount("/", routes![index, new, static_content, favicon])
    .attach(Template::fairing())
    .launch();
}