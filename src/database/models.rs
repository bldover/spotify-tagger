use diesel::prelude::*;
use crate::database::schema::{tags, songs, song_tags};
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub tag_id: i32,
    pub tag_name: String
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = songs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Song {
    pub song_id: i32,
    pub song_uri: String
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = song_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SongTag {
    pub song_id: i32,
    pub tag_id: i32
}