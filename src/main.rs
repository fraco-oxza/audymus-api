#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use dotenv::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::response::status;
use rocket::{Request, Response};
use rocket_contrib::databases::mysql;
use rocket_contrib::databases::mysql::params;
use rocket_contrib::json::Json;

use audymus_api::*;

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }
    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*")); //TODO: Change this value
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> Json<HashMap<&'static str, &'static str>> {
    let mut hash_map = HashMap::new();
    hash_map.insert("songs", "https://api.audymus.ml/songs");
    Json(hash_map)
}

#[get("/songs")]
fn get_songs(mut conn: db::AudymusDbConn) -> Json<Vec<db::Song>> {
    let selected_sounds = conn.prep_exec("SELECT * FROM song", ());

    let selected_sounds: Vec<db::Song> = selected_sounds
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, name, link, image) = mysql::from_row(row);
                    db::Song {
                        id,
                        name,
                        link,
                        image,
                    }
                })
                .collect()
        })
        .unwrap();

    return Json(selected_sounds);
}

#[get("/search/<string>")]
fn song_search(mut conn: db::AudymusDbConn, string: String) -> Json<Vec<db::Song>> {
    let selected_sounds = conn.prep_exec(
        format!("SELECT * FROM song WHERE name LIKE \'%{}%\'", string),
        (),
    );

    let selected_sounds: Vec<db::Song> = selected_sounds
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, name, link, image) = mysql::from_row(row);
                    db::Song {
                        id,
                        name,
                        link,
                        image,
                    }
                })
                .collect()
        })
        .unwrap();

    return Json(selected_sounds);
}

#[post("/songs", data = "<song>")]
fn add_song(mut conn: db::AudymusDbConn, song: Json<db::InsertableSong>) {
    for mut stmt in conn
        .prepare(r"INSERT INTO song (name, link, image) VALUES (:name, :link, :image)")
        .into_iter()
    {
        stmt.execute(params! {
            "name" => song.name.clone(),
            "link" => song.link.clone(),
            "image" => song.image.clone(),
        })
        .unwrap();
    }
}

#[options("/songs")]
fn preflight() -> status::NoContent {
    status::NoContent
}

#[get("/.well-known/acme-challenge/giJlK7g_yXzo761peP-_UbBM_0COky3XcbC4r5TSDyg")]
fn certbot_challenge() -> &'static str {
    "giJlK7g_yXzo761peP-_UbBM_0COky3XcbC4r5TSDyg.NzcYVQ2wdRYkaewHWZjUhrStm5BrF6yodFhE9cXjLgE"
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .attach(db::AudymusDbConn::fairing())
        .attach(CORS)
        .mount(
            "/",
            routes![
                get_songs,
                add_song,
                certbot_challenge,
                index,
                preflight,
                song_search
            ],
        )
        .launch();
}
