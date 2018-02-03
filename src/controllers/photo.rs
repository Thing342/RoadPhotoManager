extern crate rocket_contrib;
extern crate tera;

use super::*;

#[get("/")]
fn index(connection: DbConn) -> rocket_contrib::Template {
    let mut context = tera::Context::new();
    let photos = schema::photo::table.load::<models::Photo>(&*connection).expect("Error loading photos");

    context.add("photos", &photos);

    rocket_contrib::Template::render("base", context)
}

#[post("/", data="<photo_data>")]
fn new(connection: DbConn, photo_data: rocket::request::Form<models::NewPhoto>) -> rocket::response::Redirect {
    use schema::photo::dsl;
    use rocket::response::*;

    diesel::insert_into(dsl::photo)
        .values(photo_data.get())
        .execute(&*connection)
        .expect("Error inserting form data!");

    Redirect::to("/")
}