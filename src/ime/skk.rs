use std::collections::BTreeMap;
use std::fmt;
use std::io::Write;
use failure::Error;

use crate::dictionary::{DictionaryEntry, DictionaryEntryKind, EmitDictionary};

/// SKK 辞書エントリ
pub struct SkkDictionaryEntry {
    pub reading: String,
    pub entries: Vec<(String, DictionaryEntryKind)>,
}

impl EmitDictionary for SkkDictionaryEntry {
    fn emit(entries: &Vec<DictionaryEntry>) -> Vec<SkkDictionaryEntry> {
        let mut map = BTreeMap::<String, Vec<&DictionaryEntry>>::new();
        for entry in entries {
            if let Some(list) = map.get_mut(&entry.reading) {
                list.push(entry);
            } else {
                map.insert(entry.reading.clone(), vec![entry]);
            }
        }

        map.iter()
            .map(|(k, v)| SkkDictionaryEntry {
                reading: k.to_owned(),
                entries: v.iter().map(|e| (e.entry.to_owned(), e.kind)).collect(),
            })
            .collect()
    }
}

impl SkkDictionaryEntry {
    fn escape(text: &str) -> String {
        let replacement = vec![
            ("\\", "\\\\"),
            ("\"", "\\\""),
            ("/", "\\057"),
            (";", "\\059"),
        ];
        let find_replacement = |&(ec, _)| match text.find(ec) {
            Some(_) => true,
            None => false,
        };

        if replacement.iter().any(find_replacement) {
            let mut replaced = text.to_owned();
            for (from, to) in replacement {
                replaced = replaced.replace(from, to);
            }
            format!("(concat \"{}\")", replaced)
        } else {
            text.to_owned()
        }
    }
}

impl fmt::Display for SkkDictionaryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut candidates = vec![];
        for (word, kind) in self.entries.iter() {
            let escaped_words = SkkDictionaryEntry::escape(&word);
            let description = match kind {
                DictionaryEntryKind::Chapter(sec, cha) => {
                    format!("第{}章チャプター{}", sec, cha)
                }
                DictionaryEntryKind::Character => "キャラクター".to_owned(),
                DictionaryEntryKind::VoiceActor => "声優".to_owned(),
                DictionaryEntryKind::Unit => "ユニット".to_owned(),
                DictionaryEntryKind::Song => "楽曲".to_owned(),
                DictionaryEntryKind::Composer => "アーティスト".to_owned(),
            };
            candidates.push(format!("{};[オンゲキ] {}", escaped_words, description));
        }

        write!(f, "{} /{}/", self.reading, candidates.join("/"))
    }
}
