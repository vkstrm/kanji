use clap::{App, Arg};

extern crate kandb;

fn main() {
    let matches = App::new("kandb")
        .version("1.0")
        .about("Search for Kanji on the command line")
        .arg(
            Arg::with_name("input")
                .about("Input to search for")
                .value_name("INPUT")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("kanji")
                .about("Search using a kanji")
                .short('c')
                .takes_value(false),
        )
        .arg(
            Arg::with_name("meaning")
                .about("Search using meaning")
                .short('m')
                .takes_value(false),
        )
        .arg(
            Arg::with_name("onyomi")
                .about("Search using onyomi reading")
                .short('o')
                .takes_value(false),
        )
        .arg(
            Arg::with_name("kunyomi")
                .about("Search using kunyomi reading")
                .short('k')
                .takes_value(false),
        )
        .arg(
            Arg::with_name("verbose")
                .about("Verbose")
                .short('v')
                .takes_value(false),
        )
        .get_matches();

    kandb::interpret_commands(&matches);
}
