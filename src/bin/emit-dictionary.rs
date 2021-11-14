use env_logger;
use log::info;
use std::error::Error;
use std::{
    fs::File,
    io::{stdout, Write},
};
use structopt::StructOpt;

use ongeki_data::{
    AtokDictionaryEntry, CharactersDefinition, EmitDictionary, GeneralDefinition,
    MsimeDictionaryEntry, SkkDictionaryEntry, SongsDefinition,
};

/// オンゲキ SKK 辞書生成ツール
#[derive(StructOpt)]
struct Arguments {
    /// 出力する辞書の形式を指定 (skk, msime, atok)
    #[structopt(short = "t", long)]
    dictionary_type: String,

    /// characters.toml の位置を指定
    #[structopt(short, long, default_value = "./data/characters.toml")]
    character_definitions: String,

    /// songs.toml の位置を指定
    #[structopt(short, long, default_value = "./data/songs.toml")]
    song_definitions: String,

    /// general.toml の位置を指定
    #[structopt(short, long, default_value = "./data/general.toml")]
    general_definitions: String,

    /// 出力先
    #[structopt(default_value = "-")]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Arguments::from_args();

    run(args)
}

fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    // 定義読み込み/表示
    let generals = ongeki_data::load_general_definitions(&args.general_definitions)?;
    let characters = ongeki_data::load_character_definitions(&args.character_definitions)?;
    let songs = ongeki_data::load_song_definitions(&args.song_definitions)?;
    let entries = ongeki_data::generate_entries(&generals, &characters, &songs);
    log_information(&generals, &characters, &songs)?;

    let (mut file, mut stdo);
    let output: &mut dyn Write = if args.output == "-" {
        stdo = stdout();
        &mut stdo
    } else {
        file = File::create(args.output)?;
        &mut file
    };

    match args.dictionary_type.as_ref() {
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
            panic!("Invalid type: {}", args.dictionary_type);
        }
    }

    Ok(())
}

fn log_information(
    generals: &GeneralDefinition,
    characters: &CharactersDefinition,
    songs: &SongsDefinition,
) -> Result<(), Box<dyn Error>> {
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
