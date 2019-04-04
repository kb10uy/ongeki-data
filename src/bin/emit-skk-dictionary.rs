use clap::{App, Arg, ArgMatches};
use env_logger;
use failure::Error;
use log::info;
use std::{
    fs::File,
    io::{stdout, Read, Write},
};
use toml;

use ongeki_data::{
    generate_entries, CharactersDefinition, EmitDictionary, SkkDictionaryEntry, SongsDefinition,
};

fn main() -> Result<(), Error> {
    env_logger::init();

    let app = App::new("オンゲキ SKK 辞書生成ツール")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(
            "TOML 形式の定義ファイルから SKK 用の辞書ファイルを生成します",
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
    let mut buffer = String::new();
    let output_name = matches.value_of("OUTPUT").unwrap();
    let mut characters_file = File::open(matches.value_of("character-definitions").unwrap())?;
    let mut songs_file = File::open(matches.value_of("song-definitions").unwrap())?;
    let mut output: Box<dyn Write> = if output_name == "-" {
        Box::new(stdout())
    } else {
        Box::new(File::create(output_name)?)
    };

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
    let skk_entries = SkkDictionaryEntry::emit(&entries);

    write!(output, ";; -*- fundamental -*- ; coding: utf-8 -*-\n")?;
    write!(output, ";; okuri-ari entries.")?;
    write!(output, ";; okuri-nasi entries.")?;
    for skk_entry in skk_entries.iter() {
        write!(output, "{:#}\n", skk_entry)?;
    }

    Ok(())
}
