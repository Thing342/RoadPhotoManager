extern crate chrono;

use schema::*;

#[derive(Queryable, Identifiable, PartialEq, Debug, Serialize)]
#[table_name = "photo"]
pub struct Photo {
    pub id: i32,
    pub photo_type: String,
    pub uploaded_at: chrono::naive::NaiveDateTime,
    pub caption: String,
    pub alt_text: String
}

#[derive(Debug, Insertable, FromForm)]
#[table_name = "photo"]
pub struct NewPhoto {
    pub photo_type: String,
    pub caption: String,
    pub alt_text: String
}

#[derive(Queryable, Identifiable, PartialEq, Associations, Debug)]
#[belongs_to(Photo)]
#[table_name = "photofile"]
pub struct PhotoFile {
    pub id: i32,
    pub photo_id: i32,
    pub file_size: i32,
    pub file_mime: String,
    pub file_uri: String,
    pub photo_width: i32,
    pub photo_height: i32
}

#[derive(Queryable, Identifiable, PartialEq, Associations, Debug)]
#[belongs_to(Photo)]
#[table_name = "photolocation"]
pub struct PhotoLocation {
    pub id: i32,
    pub photo_id: i32,
    pub tm_country: Option<String>,
    pub tm_region: Option<String>,
    pub county: Option<String>,
    pub city: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>
}

#[derive(Queryable, Identifiable, PartialEq, Associations, Debug)]
#[belongs_to(Photo)]
#[table_name = "phototag"]
pub struct PhotoTag {
    pub id: i32,
    pub photo_id: i32,
    pub photo_tag: String,
    pub note: Option<String>
}