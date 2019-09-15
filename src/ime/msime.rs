use std::fmt;

use crate::dictionary::{DictionaryEntry, EmitDictionary};

/// MS-IME 辞書エントリ
pub struct MsimeDictionaryEntry {
    pub entry: String,
    pub reading: String,
    pub speech: String,
}

impl EmitDictionary for MsimeDictionaryEntry {
    fn emit(entries: &[DictionaryEntry]) -> Vec<MsimeDictionaryEntry> {
        let mut result = vec![];
        for entry in entries {
            result.push(MsimeDictionaryEntry {
                entry: entry.entry.to_owned(),
                reading: entry.reading.to_owned(),
                speech: "固有名詞".to_owned(),
            });
        }

        result
    }
}

impl fmt::Display for MsimeDictionaryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}", self.reading, self.entry, self.speech)
    }
}
