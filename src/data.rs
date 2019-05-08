use serde::Deserialize;

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

/// チャプター
#[derive(Deserialize)]
pub struct Chapter {
    /// チャプター番号
    pub number: i32,
    ///  タイトル
    pub title: String,
    /// 読み
    pub reading: String,
}

/// 章
#[derive(Deserialize)]
pub struct Section {
    /// 章番号
    pub number: i32,
    /// チャプター
    pub chapters: Vec<Chapter>,
}
