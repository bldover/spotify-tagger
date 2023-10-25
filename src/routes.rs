use actix_web::{get, post, delete, web, HttpResponse, Responder};
use crate::database::db;

#[get("/song/{song_uri}/tags")]
pub async fn get_tags_for_song(path: web::Path<String>, pool: web::Data<db::DbPool>) -> impl Responder {
    let uri = path.into_inner();
    println!("Received request to get tags for song: {uri}");
    let tags = db::query_tags_for_song_uri(&uri, pool);

    match tags {
        Ok(tag_list) => {
            println!("Successfully found tags {tag_list:?} for song {uri}");
            HttpResponse::Ok().json(tag_list)
        },
        Err(e) => {
            eprintln!("Failed to retrieve tags for song {uri} with error {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/songs/{song_uri}/tags/{tag_name}")]
pub async fn add_tag_for_song(path: web::Path<(String, String)>, pool: web::Data<db::DbPool>) -> impl Responder {
    let (uri, tag_name) = path.into_inner();
    println!("Received request to add new tag {tag_name} for song: {uri}");
    match db::add_tag_for_song_uri(&uri, &tag_name, pool) {
        Ok(_) => {
            println!("Successfully added tag {tag_name} to song {uri}");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            eprintln!("Failed to process adding tag {tag_name} to song {uri} with error {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/songs/{song_uri}/tags/{tag_name}")]
pub async fn remove_tag_from_song(path: web::Path<(String, String)>, pool: web::Data<db::DbPool>) -> impl Responder {
    let (uri, tag_name) = path.into_inner();
    println!("Received request to remove tag {tag_name} from song: {uri}");
    match db::remove_tag_from_song_uri(&uri, &tag_name, pool) {
        Ok(_) => {
            println!("Successfully removed tag {tag_name} to song {uri}");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            eprintln!("Failed to process removing tag {tag_name} from song {uri} with error {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/tags")]
pub async fn get_tags(pool: web::Data<db::DbPool>) -> impl Responder {
    println!("Received request to get all tags");
    match db::get_all_tags(pool) {
        Ok(tags) => {
            println!("Successfully retrieved all tags {tags:?}");
            HttpResponse::Ok().json(tags)
        },
        Err(e) => {
            eprintln!("Failed to retrieve all tags with error {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/tags/{tag_name}")]
pub async fn add_new_tag(path: web::Path<String>, pool: web::Data<db::DbPool>) -> impl Responder {
    let tag_name = path.into_inner();
    println!("Received request to add new tag: {tag_name}");
    match db::add_new_tag(&tag_name, pool) {
        Ok(_) => {
            println!("Successfully added new tag {tag_name}");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            eprintln!("Failed to process add for tag {tag_name} with error {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/tags/{tag_name}")]
pub async fn delete_tag(path: web::Path<String>, pool: web::Data<db::DbPool>) -> impl Responder {
    let tag_name = path.into_inner();
    println!("Received request to delete tag: {tag_name}");
    match db::delete_tag(&tag_name, pool) {
        Ok(_) => {
            println!("Successfully deleted tag: {tag_name}");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            eprintln!("Failed to process delete for tag {tag_name} with error {e}",);
            HttpResponse::InternalServerError().finish()
        }
    }
}