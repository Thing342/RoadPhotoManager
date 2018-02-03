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

#[derive(Debug, Deserialize)]
pub struct NewPhotoUpload {
    pub photo: NewPhoto,
    pub loc: NewPhotoLocation,
    pub file: NewPhotoFile,
    pub tags: Vec<NewPhotoTag>
}

impl NewPhotoUpload {
    pub fn set_photo_id(&mut self, photo_id: i32) {
        self.loc.photo_id = photo_id;
        self.file.photo_id = photo_id;

        for tag in self.tags.iter_mut() {
            tag.photo_id = photo_id;
        }
    }
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "photo"]
pub struct NewPhoto {
    pub photo_type: String,
    pub caption: String,
    pub alt_text: String,
}

#[derive(Queryable, Identifiable, Serialize, PartialEq, Associations, Debug)]
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

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "photofile"]
pub struct NewPhotoFile {
    #[serde(default = "default_val")]
    pub photo_id: i32,
    pub file_size: i32,
    pub file_mime: String,
    pub file_uri: String,
    pub photo_width: i32,
    pub photo_height: i32
}

#[derive(Queryable, Identifiable, Serialize, PartialEq, Associations, Debug)]
#[belongs_to(Photo)]
#[table_name = "photolocation"]
pub struct PhotoLocation {
    pub id: i32,
    pub photo_id: i32,
    pub tm_country: Option<String>,
    pub tm_region: Option<String>,
    pub county: Option<String>,
    pub city: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "photolocation"]
pub struct NewPhotoLocation {
    #[serde(default = "default_val")]
    pub photo_id: i32,
    pub tm_country: Option<String>,
    pub tm_region: Option<String>,
    pub county: Option<String>,
    pub city: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>
}

#[derive(Queryable, Identifiable, PartialEq, Serialize, Associations, Debug)]
#[belongs_to(Photo)]
#[table_name = "phototag"]
pub struct PhotoTag {
    pub id: i32,
    pub photo_id: i32,
    pub photo_tag: String,
    pub note: Option<String>
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "phototag"]
pub struct NewPhotoTag {
    #[serde(default = "default_val")]
    pub photo_id: i32,
    pub photo_tag: String,
    pub note: Option<String>
}

fn default_val() -> i32 { 0 }