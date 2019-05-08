use std::fmt;

use crate::dictionary::{DictionaryEntry, DictionaryEntryKind, EmitDictionary};

/// ATOK 辞書エントリ
pub struct AtokDictionaryEntry {
    pub entry: String,
    pub reading: String,
    pub speech: String,
}

impl EmitDictionary for AtokDictionaryEntry {
    fn emit(entries: &Vec<DictionaryEntry>) -> Vec<AtokDictionaryEntry> {
        let mut result = vec![];
        for entry in entries {
            result.push(AtokDictionaryEntry {
                entry: entry.entry.to_owned(),
                reading: entry.reading.to_owned(),
                speech: match entry.kind {
                    DictionaryEntryKind::Character
                    | DictionaryEntryKind::VoiceActor
                    | DictionaryEntryKind::Composer => "固有人他",
                    DictionaryEntryKind::Unit => "固有組織",
                    DictionaryEntryKind::Song | DictionaryEntryKind::Chapter(_, _) => "固有一般",
                }
                .to_owned(),
            });
        }

        result
    }
}

impl fmt::Display for AtokDictionaryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}", self.reading, self.entry, self.speech)
    }
}
