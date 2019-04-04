use env_logger;
use failure::Error;
use log::info;
use std::fs::File;
use std::io::Read;
use toml;

use ongeki_data::{
    generate_entries, CharactersDefinition, EmitDictionary, SkkDictionaryEntry, SongsDefinition,
};

fn main() -> Result<(), Error> {
    env_logger::init();

    let mut buffer = String::new();
    let mut characters_file = File::open("./data/characters.toml")?;
    let mut songs_file = File::open("./data/songs.toml")?;

    characters_file.read_to_string(&mut buffer)?;
    let characters: CharactersDefinition = toml::from_str(&buffer)?;

    songs_file.read_to_string(&mut buffer)?;
    let songs: SongsDefinition = toml::from_str(&buffer)?;

    info!(
        "Character definitions loaded (updated on {})",
        characters.updated_at
    );
    info!("Characters: {}", characters.characters.len());
    info!("Units: {}", characters.units.len());
    info!("Song definitions loaded (updated on {})", songs.updated_at);
    info!("Songs: {}", songs.songs.len());

    let entries = generate_entries(&characters, &songs);
    for entry in entries.iter() {
        println!("{:#}", entry);
    }

    let skk_entries = SkkDictionaryEntry::emit(&entries);
    for skk_entry in skk_entries.iter() {
        println!("{:#}", skk_entry);
    }

    Ok(())
}
