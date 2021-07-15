use clap::{App, Arg};

use kanji;

fn main() {
    let matches = App::new("ngo")
        .subcommand(App::new("kana")
            .about("Transform text into kana")
            .arg(
                Arg::new("input")
                    .about("Input to transform")
                    .required(true)
                    .takes_value(true)
            )
            .arg(
                Arg::new("hiragana")
                    .about("To hiragana")
                    .long("hiragana")
                    .short('h')   
            )
            .arg(
                Arg::new("katakana")
                    .about("To katakana")
                    .long("katakana")
                    .short('k')   
            )
        )
        .subcommand(App::new("kanji")
            .about("Look up Kanji meaning and readings")
            .arg(
                Arg::new("input")
                    .about("Input to search for")
                    .takes_value(true)
            )
            .arg(
                Arg::new("character")
                    .about("Search using a kanji")
                    .short('c')
                    .long("character")
                    .requires("input")
                    .takes_value(false),
            )
            .arg(
                Arg::new("meaning")
                    .about("Search using meaning")
                    .short('m')
                    .long("meaning")
                    .requires("input")
                    .takes_value(false),
            )
            .arg(
                Arg::new("substring")
                    .about("Search meaning using substring")
                    .requires("input").requires("meaning")
                    .short('s')
                    .long("substring")
                    .takes_value(false)
            )
            .arg(
                Arg::new("onyomi")
                    .about("Search using onyomi reading")
                    .short('o')
                    .long("onyomi")
                    .requires("input")
                    .takes_value(false),
            )
            .arg(
                Arg::new("kunyomi")
                    .about("Search using kunyomi reading")
                    .short('k')
                    .long("kunyomi")
                    .requires("input")
                    .takes_value(false),
            )
        )
        .get_matches();

    kanji::handle_arguments(&matches);
}
