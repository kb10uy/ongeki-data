use std::fs::File;
use std::io::Read;
use failure::Error;
use serde::Deserialize;
use toml;

use crate::data::{Character, Unit, Song};

/// キャラクター・ユニットの定義ファイル (characters.toml)
#[derive(Deserialize)]
pub struct CharactersDefinition {
    pub updated_at: String,
    pub characters: Vec<Character>,
    pub units: Vec<Unit>,
}

/// 楽曲の定義ファイル (songs.toml)
#[derive(Deserialize)]
pub struct SongsDefinition {
    pub updated_at: String,
    pub songs: Vec<Song>,
}


/// 指定されたパスのファイルからキャラクター定義を読み込む
pub fn load_character_definitions(path: &str) -> Result<CharactersDefinition, Error> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let result: CharactersDefinition = toml::from_str(&buffer)?;
    Ok(result)
}

/// 指定されたパスのファイルから楽曲定義を読み込む
pub fn load_song_definitions(path: &str) -> Result<SongsDefinition, Error> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let result: SongsDefinition = toml::from_str(&buffer)?;
    Ok(result)
}
