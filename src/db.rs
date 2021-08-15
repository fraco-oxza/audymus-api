use rocket_contrib::databases::mysql;
use serde::{Deserialize, Serialize};

#[database("audymus_db")]
pub struct AudymusDbConn(mysql::Conn);

#[derive(Serialize, Deserialize)]
pub struct Song {
    pub id: u64,
    pub name: String,
    pub link: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsertableSong {
    pub name: String,
    pub link: String,
    pub image: String,
}
