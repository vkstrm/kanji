use clap::{App, Arg};

use kanji;

fn main() {
    let matches = App::new("kanji")
        .version("1.0")
        .about("Lookup Kanji meaning and readings")
        .arg(
            Arg::with_name("input")
                .about("Input to search for")
                .value_name("INPUT")
                .number_of_values(1)
                .index(1),
        )
        .arg(
            Arg::with_name("kanji")
                .about("Search using a kanji")
                .short('c')
                .long("character")
                .requires("input")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("meaning")
                .about("Search using meaning")
                .short('m')
                .long("meaning")
                .requires("input")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("substring")
                .about("Search meaning using substring")
                .requires("input").requires("meaning")
                .short('s')
                .long("substring")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("onyomi")
                .about("Search using onyomi reading")
                .short('o')
                .long("onyomi")
                .requires("input")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("kunyomi")
                .about("Search using kunyomi reading")
                .short('k')
                .long("kunyomi")
                .requires("input")
                .takes_value(false),
        )
        .get_matches();

    kanji::handle_arguments(&matches);
}
