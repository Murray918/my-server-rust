use serde::{Deserialize, Serialize};

/// Schema for creating or updating a player
#[derive(Serialize, Deserialize, Debug)]
pub struct GameSchema {
    pub name: String,
    pub creator: String,
    pub plays: i32,
}
