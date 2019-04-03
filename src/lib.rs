use serde::Deserialize;

#[derive(Deserialize)]
pub struct VoiceActor {
    pub name: Vec<String>,
    pub reading: Vec<String>,
}

#[derive(Deserialize)]
pub struct Character {
    pub name: Vec<String>,
    pub reading: Vec<String>,
    pub voice_actor: VoiceActor,
}

#[derive(Deserialize)]
pub struct Unit {
    pub name: String,
    pub reading: String,
}

#[derive(Deserialize)]
pub struct Composer {
    pub name: String,
    pub reading: String,
}

#[derive(Deserialize)]
pub struct Song {
    pub title: String,
    pub reading: String,
    pub composer: Composer,
}

#[derive(Deserialize)]
pub struct CharactersDefinition {
    pub updated_at: String,
    pub characters: Vec<Character>,
    pub units: Vec<Unit>,
}

#[derive(Deserialize)]
pub struct SongsDefinition {
    pub updated_at: String,
    pub songs: Vec<Song>,
}
