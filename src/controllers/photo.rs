extern crate rocket_contrib;
extern crate tera;

use super::*;

type TemplateOr404 = Result<rocket_contrib::Template, rocket::response::status::NotFound<String>>;

#[get("/")]
fn index(connection: DbConn) -> rocket_contrib::Template {
    let mut context = tera::Context::new();
    let photos = schema::photo::table.load::<models::Photo>(&*connection)
        .expect("Error loading photos");
    let files = models::PhotoFile::belonging_to(&photos)
        .load::<models::PhotoFile>(&*connection)
        .expect("Error loading photo file info")
        .grouped_by(&photos);

    let content = photos.into_iter().zip(files).collect::<Vec<_>>();

    context.add("content", &content);

    rocket_contrib::Template::render("photo_list", context)
}

#[post("/", data="<photo_json>")]
fn new<'a>(connection: DbConn, photo_json: rocket_contrib::Json<models::NewPhotoUpload>) -> rocket::Response<'a> {
    let mut upload_data: models::NewPhotoUpload = photo_json.into_inner();

    let newphoto: models::Photo = diesel::insert_into(schema::photo::table)
        .values(&upload_data.photo)
        .get_result(&*connection)
        .expect("Error inserting photo!");

    upload_data.set_photo_id(newphoto.id);

    diesel::insert_into(schema::photofile::table)
        .values(&upload_data.file)
        .execute(&*connection)
        .expect("Error inserting photo file!");

    diesel::insert_into(schema::photolocation::table)
        .values(&upload_data.loc)
        .execute(&*connection)
        .expect("Error inserting photo location!");

    diesel::insert_into(schema::phototag::table)
        .values(&upload_data.tags)
        .execute(&*connection)
        .expect("Error inserting photo tags!");

    use rocket::http::{Status, ContentType};

    rocket::Response::build()
        .status(Status::Created)
        .header(ContentType::Plain)
        .finalize()
}

#[post("/delete/<id>")]
fn delete(connection: DbConn, id: i32) -> Result<rocket::response::status::NoContent, rocket::response::status::NotFound<String>> {
    fn query(connection: DbConn, id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(schema::photo::table.find(id)).execute(&*connection)?;
        Ok(())
    }

    match query(connection, id) {
        Ok(_) => Ok(rocket::response::status::NoContent),
        Err(err) => Err(rocket::response::status::NotFound(err.to_string()))
    }
}

#[get("/<id>")]
fn show(connection: DbConn, id: i32) -> TemplateOr404 {
    fn query(connection: DbConn, id: i32) -> Result<tera::Context, diesel::result::Error> {
        let mut context = tera::Context::new();
        let photo: models::Photo = schema::photo::table.find(id)
            .get_result::<models::Photo>(&*connection)?;

        let loc: models::PhotoLocation = models::PhotoLocation::belonging_to(&photo).first(&*connection)?;
        let file: models::PhotoFile = models::PhotoFile::belonging_to(&photo).first(&*connection)?;
        let tags: Vec<models::PhotoTag> = models::PhotoTag::belonging_to(&photo).load(&*connection)?;

        context.add("photo", &photo);
        context.add("location", &loc);
        context.add("file", &file);
        context.add("tags", &tags);

        Ok(context)
    }

    match query(connection, id) {
        Ok(context) => Ok(rocket_contrib::Template::render("photo", context)),
        Err(err) => Err(rocket::response::status::NotFound(err.to_string()))
    }
}