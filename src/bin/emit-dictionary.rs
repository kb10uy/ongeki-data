use clap::{App, Arg, ArgMatches};
use env_logger;
use failure::Error;
use log::info;
use std::{
    fs::File,
    io::{stdout, Write},
};

use ongeki_data::{
    AtokDictionaryEntry, CharactersDefinition, EmitDictionary, GeneralDefinition,
    MsimeDictionaryEntry, SkkDictionaryEntry, SongsDefinition,
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
            Arg::with_name("dictionary-type")
                .short("t")
                .long("type")
                .help("出力する辞書の形式を指定 (skk, msime, atok)")
                .required(true)
                .takes_value(true)
                .value_name("TYPE"),
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
        )
        .arg(
            Arg::with_name("general-definitions")
                .short("g")
                .long("general")
                .help("general.toml の位置を指定")
                .takes_value(true)
                .value_name("FILE")
                .default_value("./data/general.toml"),
        );
    let matches = app.get_matches();

    run(&matches)
}

fn run(matches: &ArgMatches) -> Result<(), Error> {
    // 引数
    let output_name = matches.value_of("OUTPUT").unwrap();
    let output_type = matches.value_of("type").unwrap();
    let generals_filename = matches.value_of("general-definitions").unwrap();
    let characters_filename = matches.value_of("character-definitions").unwrap();
    let songs_filename = matches.value_of("song-definitions").unwrap();

    // 定義読み込み/表示
    let generals = ongeki_data::load_general_definitions(generals_filename)?;
    let characters = ongeki_data::load_character_definitions(characters_filename)?;
    let songs = ongeki_data::load_song_definitions(songs_filename)?;
    let entries = ongeki_data::generate_entries(&generals, &characters, &songs);
    log_information(&generals, &characters, &songs)?;

    let (mut file, mut stdo);
    let output: &mut dyn Write = if output_name == "-" {
        stdo = stdout();
        &mut stdo
    } else {
        file = File::create(output_name)?;
        &mut file
    };

    match output_type {
        "skk" => {
            let skk_entries = SkkDictionaryEntry::emit(&entries);
            write!(output, ";; -*- fundamental -*- ; coding: utf-8 -*-\n")?;
            write!(output, ";; okuri-ari entries.\n")?;
            write!(output, ";; okuri-nasi entries.\n")?;
            for skk_entry in skk_entries.iter() {
                write!(output, "{:#}\n", skk_entry)?;
            }
        }
        "msime" => {
            let msime_entries = MsimeDictionaryEntry::emit(&entries);
            output.write_all(&[0xff, 0xfe])?;
            ongeki_data::write_as_utf16(output, &format!("!Microsoft IME Dictionary Tool 98\n"))?;
            ongeki_data::write_as_utf16(output, &format!("!--------------------------------\n"))?;
            ongeki_data::write_as_utf16(
                output,
                &format!("! characters: {}\n", characters.updated_at),
            )?;
            ongeki_data::write_as_utf16(output, &format!("! songs: {}\n", songs.updated_at))?;
            ongeki_data::write_as_utf16(output, &format!("!--------------------------------\n"))?;
            for msime_entry in msime_entries.iter() {
                ongeki_data::write_as_utf16(output, &format!("{:#}\n", msime_entry))?;
            }
        }
        "atok" => {
            let atok_entries = AtokDictionaryEntry::emit(&entries);
            output.write_all(&[0xff, 0xfe])?;
            ongeki_data::write_as_utf16(output, &format!("!!ATOK_TANGO_TEXT_HEADER_1\n"))?;
            ongeki_data::write_as_utf16(output, &format!("!!--------------------------------\n"))?;
            ongeki_data::write_as_utf16(
                output,
                &format!("!! characters: {}\n", characters.updated_at),
            )?;
            ongeki_data::write_as_utf16(output, &format!("!! songs: {}\n", songs.updated_at))?;
            ongeki_data::write_as_utf16(output, &format!("!!--------------------------------\n"))?;
            for atok_entry in atok_entries.iter() {
                ongeki_data::write_as_utf16(output, &format!("{:#}\n", atok_entry))?;
            }
        }
        _ => {
            panic!("Invalid type: {}", output_type);
        }
    }

    Ok(())
}

fn log_information(
    generals: &GeneralDefinition,
    characters: &CharactersDefinition,
    songs: &SongsDefinition,
) -> Result<(), Error> {
    info!(
        "General definitions loaded (updated on {})",
        generals.updated_at
    );
    info!("Sections: {}", generals.sections.len());
    info!(
        "Character definitions loaded (updated on {})",
        characters.updated_at
    );
    info!("Characters: {}", characters.characters.len());
    info!("Units: {}", characters.units.len());
    info!("Song definitions loaded (updated on {})", songs.updated_at);
    info!("Songs: {}", songs.songs.len());
    Ok(())
}
