mod data;
mod dictionary;
mod io;
mod ime {
    pub mod atok;
    pub mod msime;
    pub mod skk;
}

pub use crate::{
    data::{Character, Composer, Song, Unit, VoiceActor},
    dictionary::{generate_entries, DictionaryEntry, DictionaryEntryKind, EmitDictionary},
    ime::{atok::AtokDictionaryEntry, msime::MsimeDictionaryEntry, skk::SkkDictionaryEntry},
    io::{
        load_character_definitions, load_general_definitions, load_song_definitions,
        write_as_utf16, CharactersDefinition, GeneralDefinition, SongsDefinition,
    },
};
