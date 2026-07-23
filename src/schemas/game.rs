use serde::{Deserialize, Serialize};

/// Schema for creating or updating a player
#[derive(Serialize, Deserialize, Debug)]
pub struct GameSchema {
    pub name: String,
    pub creator: String,
    pub plays: i32,
}

/// Schema for updating an existing note
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGameSchema {
    pub name: Option<String>,
    pub creator: Option<String>,
    pub plays: Option<i32>,
}
