// @generated automatically by Diesel CLI.

diesel::table! {
    song_tags (song_id, tag_id) {
        song_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    songs (song_id) {
        song_id -> Int4,
        song_uri -> Text,
    }
}

diesel::table! {
    tags (tag_id) {
        tag_id -> Int4,
        tag_name -> Text,
    }
}

diesel::joinable!(song_tags -> songs (song_id));
diesel::joinable!(song_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    song_tags,
    songs,
    tags,
);
