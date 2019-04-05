use clap::{App, Arg, ArgMatches};
use env_logger;
use failure::Error;
use log::info;
use std::{
    fs::File,
    io::{stdout, Write},
};

use ongeki_data::{EmitDictionary, AtokDictionaryEntry};

fn main() -> Result<(), Error> {
    env_logger::init();

    let app = App::new("オンゲキ ATOK 辞書生成ツール")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(
            "TOML 形式の定義ファイルから ATOK 用の辞書ファイルを生成します",
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("出力先")
                .required(true)
                .index(1)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("character-definitions")
                .short("c")
                .long("characters")
                .help("characters.toml の位置を指定")
                .takes_value(true)
                .value_name("FILE")
                .default_value("./data/characters.toml"),
        )
        .arg(
            Arg::with_name("song-definitions")
                .short("s")
                .long("songs")
                .help("songs.toml の位置を指定")
                .takes_value(true)
                .value_name("FILE")
                .default_value("./data/songs.toml"),
        );
    let matches = app.get_matches();

    run(&matches)
}

fn run(matches: &ArgMatches) -> Result<(), Error> {
    let output_name = matches.value_of("OUTPUT").unwrap();
    let mut output: Box<dyn Write> = if output_name == "-" {
        Box::new(stdout())
    } else {
        Box::new(File::create(output_name)?)
    };

    let characters = ongeki_data::load_character_definitions(
        matches.value_of("character-definitions").unwrap(),
    )?;
    let songs = ongeki_data::load_song_definitions(matches.value_of("song-definitions").unwrap())?;

    info!(
        "Character definitions loaded (updated on {})",
        characters.updated_at
    );
    info!("Characters: {}", characters.characters.len());
    info!("Units: {}", characters.units.len());
    info!("Song definitions loaded (updated on {})", songs.updated_at);
    info!("Songs: {}", songs.songs.len());

    let entries = ongeki_data::generate_entries(&characters, &songs);
    let atok_entries = AtokDictionaryEntry::emit(&entries);

    write!(output, "!!ATOK_TANGO_TEXT_HEADER_1\n")?;
    write!(output, "!!--------------------------------\n")?;
    write!(output, "!! characters: {}\n", characters.updated_at)?;
    write!(output, "!! songs: {}\n", songs.updated_at)?;
    write!(output, "!!--------------------------------\n")?;
    for atok_entry in atok_entries.iter() {
        write!(output, "{:#}\n", atok_entry)?;
    }

    Ok(())
}
