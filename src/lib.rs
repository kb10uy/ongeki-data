use std::fmt;
use serde::Deserialize;

mod skk;

pub use crate::skk::SkkDictionaryEntry;

/// キャラクター声優
#[derive(Deserialize)]
pub struct VoiceActor {
    /// 名前
    pub name: Vec<String>,
    /// 読み
    pub reading: Vec<String>,
    /// 名前の順が反転している
    pub inverted: Option<bool>,
}

/// キャラクター
#[derive(Deserialize)]
pub struct Character {
    /// 名前
    pub name: Vec<String>,
    /// 読み
    pub reading: Vec<String>,
    /// 名前の順が反転している
    pub inverted: Option<bool>,
    /// 声優
    pub voice_actor: VoiceActor,
}

/// キャラクターユニット
#[derive(Deserialize)]
pub struct Unit {
    /// 名前
    pub name: String,
    pub reading: String,
}

/// 楽曲アーティスト
#[derive(Deserialize)]
pub struct Composer {
    /// 名前
    pub name: String,
    /// 読み
    pub reading: String,
}

/// 楽曲
#[derive(Deserialize)]
pub struct Song {
    /// 名前
    pub title: String,
    /// 読み
    pub reading: String,
    /// アーティスト
    pub composer: Composer,
}

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

// TODO: これ直接格納してもいいのでは?
/// 辞書エントリの種類
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum DictionaryEntryKind {
    /// キャラクター
    Character,
    /// 声優
    VoiceActor,
    /// ユニット
    Unit,
    /// 曲名
    Song,
    /// コンポーザー
    Composer,
}

/// 単一の辞書エントリ
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DictionaryEntry {
    /// エントリ種類
    pub kind: DictionaryEntryKind,
    /// 見出し語
    pub entry: String,
    /// 読み
    pub reading: String,
}

impl fmt::Display for DictionaryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{:?}] {}", self.entry, self.kind, self.reading)?;
        Ok(())
    }
}

/// 辞書エントリを出力可能
pub trait EmitDictionary
where
    Self: Sized,
{
    fn emit(entries: &Vec<DictionaryEntry>) -> Vec<Self>;
}

/// 抽象的な辞書エントリを生成する
pub fn generate_entries(
    characters: &CharactersDefinition,
    songs: &SongsDefinition,
) -> Vec<DictionaryEntry> {
    let mut result = vec![];

    // キャラクター名
    for character in characters.characters.iter() {
        // 部分別
        let zipped = character.name.iter().zip(character.reading.iter());
        for (name, reading) in zipped {
            result.push(DictionaryEntry {
                kind: DictionaryEntryKind::Character,
                entry: name.to_owned(),
                reading: reading.to_owned(),
            });
        }

        if character.name.len() == 1 {
            continue;
        }

        // フルネーム
        let full = if let Some(true) = character.inverted {
            character.name.iter().rev().fold(String::new(), |a, p| a + p)
        } else {
            character.name.join("")
        };
        let full_reading = if let Some(true) = character.inverted {
            character.reading.iter().rev().fold(String::new(), |a, p| a + p)
        } else {
            character.reading.join("")
        };
        result.push(DictionaryEntry {
            kind: DictionaryEntryKind::Character,
            entry: full,
            reading: full_reading,
        });

        // 声優フルネーム
        let va_full = if let Some(true) = character.voice_actor.inverted {
            character.voice_actor.name.iter().rev().fold(String::new(), |a, p| a + p)
        } else {
            character.voice_actor.name.join("")
        };
        let va_full_reading = if let Some(true) = character.voice_actor.inverted {
            character.voice_actor.reading.iter().rev().fold(String::new(), |a, p| a + p)
        } else {
            character.voice_actor.reading.join("")
        };
        result.push(DictionaryEntry {
            kind: DictionaryEntryKind::VoiceActor,
            entry: va_full,
            reading: va_full_reading,
        });
    }

    // ユニット
    for unit in characters.units.iter() {
        result.push(DictionaryEntry {
            kind: DictionaryEntryKind::Unit,
            entry: unit.name.to_owned(),
            reading: unit.reading.to_owned(),
        });
    }

    for song in songs.songs.iter() {
        // 楽曲
        result.push(DictionaryEntry {
            kind: DictionaryEntryKind::Song,
            entry: song.title.to_owned(),
            reading: song.reading.to_owned(),
        });

        // アーティスト
        result.push(DictionaryEntry {
            kind: DictionaryEntryKind::Composer,
            entry: song.composer.name.to_owned(),
            reading: song.composer.reading.to_owned(),
        });
    }

    result.sort();
    result.dedup();
    result
}
