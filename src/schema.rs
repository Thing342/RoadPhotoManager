table! {
    photo (id) {
        id -> Int4,
        photo_type -> Varchar,
        uploaded_at -> Timestamp,
        caption -> Text,
        alt_text -> Text,
    }
}

table! {
    photofile (id) {
        id -> Int4,
        photo_id -> Int4,
        file_size -> Int4,
        file_mime -> Varchar,
        file_uri -> Text,
        photo_width -> Int4,
        photo_height -> Int4,
    }
}

table! {
    photolocation (id) {
        id -> Int4,
        photo_id -> Int4,
        tm_country -> Nullable<Varchar>,
        tm_region -> Nullable<Varchar>,
        county -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        lat -> Nullable<Float8>,
        lng -> Nullable<Float8>,
    }
}

table! {
    phototag (id) {
        id -> Int4,
        photo_id -> Int4,
        photo_tag -> Varchar,
        note -> Nullable<Text>,
    }
}

joinable!(photofile -> photo (photo_id));
joinable!(photolocation -> photo (photo_id));
joinable!(phototag -> photo (photo_id));

allow_tables_to_appear_in_same_query!(
    photo,
    photofile,
    photolocation,
    phototag,
);
