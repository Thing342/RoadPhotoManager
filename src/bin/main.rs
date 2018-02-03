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

fn main() {
  rocket::ignite()
    .manage(create_db_pool()) // Register connection pool
    .mount("/", routes![index])
    .attach(Template::fairing())
    .launch();
}