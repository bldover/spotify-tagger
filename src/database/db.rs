use diesel::{prelude::*, r2d2, pg::PgConnection};
use std::{env, thread};
use actix_web::web;

use crate::database::models::*;
use crate::database::schema::{songs, song_tags, tags};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn initialize_db_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Database URL should be valid PostgreSQL URL")
}

pub fn query_tags_for_song_uri(uri: &str, pool: web::Data<DbPool>) -> Result<Vec<String>, diesel::result::Error> {

    let connection = &mut pool.get().unwrap();
    let tag_rows = songs::dsl::songs
        .inner_join(song_tags::dsl::song_tags.inner_join(tags::dsl::tags))
        .filter(songs::dsl::song_uri.eq(uri))
        .select(Tag::as_select())
        .load(connection)?;
    let tag_names = tag_rows.into_iter()
        .map(|tag| tag.tag_name)
        .collect::<Vec<_>>();

    Ok(tag_names)
}

// adds tag association to the song. will create a new tag if this one does not already exist
pub fn add_tag_for_song_uri(uri: &str, tag_name: &str, pool: web::Data<DbPool>) -> Result<(), diesel::result::Error> {

    // create song if needed
    let connection = &mut pool.get().unwrap();
    let song_rows = songs::dsl::songs
        .filter(songs::dsl::song_uri.eq(uri))
        .select(Song::as_select())
        .load(connection)?;
    let mut song_row = song_rows.get(0);
    let new_song_row;
    if song_row.is_none() {
        println!("Creating new song with URI {uri}");
        new_song_row = diesel::insert_into(songs::dsl::songs)
            .values(songs::dsl::song_uri.eq(uri))
            .returning(Song::as_returning())
            .load(connection)?;
        song_row = new_song_row.get(0);
    }

    // create tag if needed
    let tag_rows = tags::dsl::tags
        .filter(tags::dsl::tag_name.eq(tag_name))
        .select(Tag::as_select())
        .load(connection)?;
    let mut tag_row = tag_rows.get(0);
    let new_tag_row;
    if tag_row.is_none() {
        println!("Creating new tag: {tag_name}");
        new_tag_row = diesel::insert_into(tags::dsl::tags)
            .values(&tags::dsl::tag_name.eq(tag_name))
            .returning(Tag::as_returning())
            .load(connection)?;
        tag_row = new_tag_row.get(0);
    }

    // create link if needed
    let song_id = song_row.unwrap().song_id;
    let tag_id = tag_row.unwrap().tag_id;
    let existing_row = song_tags::dsl::song_tags
        .filter(song_tags::dsl::song_id.eq(song_id))
        .filter(song_tags::dsl::tag_id.eq(tag_id))
        .select(SongTag::as_select())
        .load(connection)?;
    if !existing_row.is_empty() {
        println!("Tag {tag_name} already exists for song {uri}");
        return Ok(());
    }
    diesel::insert_into(song_tags::dsl::song_tags)
        .values((&song_tags::dsl::song_id.eq(song_id),
                 &song_tags::dsl::tag_id.eq(tag_id)))
        .execute(connection)?;

    Ok(())
}

// removes only this tag from the song and deletes the song if this is the only tag
pub fn remove_tag_from_song_uri(uri: &str, tag_name: &str, pool: web::Data<DbPool>) -> Result<(), diesel::result::Error> {

    let connection = &mut pool.get().unwrap();

    let song_id_subquery = songs::dsl::songs
        .filter(songs::dsl::song_uri.eq(uri))
        .select(songs::dsl::song_id)
        .into_boxed();
    let tag_id_subquery = tags::dsl::tags
        .filter(tags::dsl::tag_name.eq(tag_name))
        .select(tags::dsl::tag_id)
        .into_boxed();
    diesel::delete(song_tags::dsl::song_tags
        .filter(song_tags::dsl::song_id.eq_any(song_id_subquery))
        .filter(song_tags::dsl::tag_id.eq_any(tag_id_subquery)))
        .execute(connection)?;

    let tags = query_tags_for_song_uri(uri, pool)?;
    if tags.is_empty() {
        diesel::delete(songs::dsl::songs
            .filter(songs::dsl::song_id.eq_any(song_id_subquery)))
            .execute(connection)?;
        println!("After removing tag {tag_name}, song {uri} had no more tags and was removed");
    }

    Ok(())
}

pub fn get_all_tags(pool: web::Data<DbPool>) -> Result<Vec<String>, diesel::result::Error> {

    let connection = &mut pool.get().unwrap();
    let tag_rows = tags::dsl::tags
        .select(Tag::as_select())
        .load(connection)?;
    let tag_names = tag_rows.into_iter()
        .map(|tag| tag.tag_name)
        .collect::<Vec<_>>();

    Ok(tag_names)
}

pub fn add_new_tag(tag: &str, pool: web::Data<DbPool>) -> Result<(), diesel::result::Error> {
    let connection = &mut pool.get().unwrap();
    let inserted_row_count = diesel::insert_into(tags::dsl::tags)
        .values(&tags::dsl::tag_name.eq(tag))
        .on_conflict_do_nothing()
        .execute(connection)?;
    if inserted_row_count != 1 {
        println!("Inserted incorrect number of rows ({inserted_row_count}) when adding new tag {tag}");
    }

    Ok(())
}

pub fn delete_tag(tag: &str, pool: web::Data<DbPool>) -> Result<(), diesel::result::Error> {

    // finds tag_id for this tag from the tags table
    let connection = &mut pool.get().unwrap();
    let tag_row = tags::dsl::tags
        .filter(tags::dsl::tag_name.eq(tag))
        .select(Tag::as_select())
        .load(connection)?;
    let tag_id = match tag_row.get(0) {
        Some(row) => row.tag_id,
        None => 0
    };

    // first delete this tag's records from song_tags, then the tag itself from tags
    let song_count = diesel::delete(song_tags::dsl::song_tags
        .filter(song_tags::dsl::tag_id.eq(tag_id)))
        .execute(connection)?;
    println!("Removed tag {tag} association for {song_count} songs");
    let deleted_row_count = diesel::delete(tags::dsl::tags
        .filter(tags::dsl::tag_name.eq(tag)))
        .execute(connection)?;
    if deleted_row_count != 1 {
        println!("Did not find row to delete for tag: {tag}");
    }
    thread::spawn(|| cleanup_songs(pool));

    Ok(())
}

// after any deletion, it's possible that some songs exist in the songs table
// which have no associated tags. we remove those here, so this method should
// be called following the removal of any tag
fn cleanup_songs(pool: web::Data<DbPool>) -> Result<(), diesel::result::Error> {

    let connection = &mut pool.get().unwrap();
    let songs_to_delete_subquery = songs::dsl::songs
        .left_join(song_tags::dsl::song_tags)
        .filter(song_tags::dsl::tag_id.is_null())
        .select(songs::dsl::song_id)
        .distinct()
        .into_boxed();
    let deleted_songs_count = diesel::delete(songs::dsl::songs
        .filter(songs::dsl::song_id.eq_any(songs_to_delete_subquery)))
        .execute(connection)?;

    println!("Cleaned up {deleted_songs_count} songs");
    Ok(())
}
