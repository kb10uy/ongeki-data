mod data;
mod dictionary;
mod io;
mod ime {
    pub mod atok;
    pub mod skk;
}


pub use crate::{
    data::{Character, Composer, Song, Unit, VoiceActor},
    dictionary::{generate_entries, DictionaryEntry, DictionaryEntryKind, EmitDictionary},
    io::{
        load_character_definitions, load_song_definitions, CharactersDefinition, SongsDefinition,
    },
    ime::{skk::SkkDictionaryEntry, atok::AtokDictionaryEntry},
};
