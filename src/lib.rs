use clap::{ArgMatches};

mod dictionary;
mod kana;
mod printer;
mod custom_error;

pub struct Row {
    pub character: String,
    pub kunyomi: Vec<String>,
    pub onyomi: Vec<String>,
    pub meaning: Vec<String>
}

pub fn handle_arguments(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("kanji", args)) => {
            handle_kanji(&args);
        },
        Some(("kana", args)) => {
            handle_kana(&args);
        },
        _ => {}
    }
}

fn handle_kanji(matches: &clap::ArgMatches) {
    match matches.value_of("input") {
        Some(input) => {
            match matches.is_present("meaning") {
                true => {
                    if matches.is_present("substring") {
                        match dictionary::by_meaning_substring(input) {
                            Ok(rows) => printer::print(&rows),
                            Err(why) => match_error(why)
                        }
                    } else {
                        match dictionary::by_meaning(input) {
                            Ok(rows) => printer::print(&rows),
                            Err(why) => match_error(why)
                        }
                    }
                }, _ => {}
            }
            match matches.is_present("onyomi") {
                true => {
                    match dictionary::by_onyomi(input) {
                        Ok(rows) => printer::print(&rows),
                        Err(why) => match_error(why)
                    }
                }, _ => {}
            }
            match matches.is_present("kunyomi") {
                true => {
                    match dictionary::by_kunyomi(input) {
                        Ok(rows) => printer::print(&rows),
                        Err(why) => match_error(why)
                    }
                }, _ => {}
            }
            match matches.is_present("kanji") {
                true => {
                    match dictionary::by_character(input) {
                       Ok(rows) => printer::print(&rows),
                       Err(why) => match_error(why)
                    }
                }, _ => {}
            }
        
            // TODO query on both on and kun and get distinct result back
        }
        None => {}
    }
}

fn handle_kana(args: &clap::ArgMatches) {
    if let Some(input) = args.value_of("input") {
        if args.is_present("katakana") {
            match kana::str_to_katakana(input) {
                Ok(katakana) => {
                    println!("{}", katakana);
                },
                Err(why) => match_error(why)
            }
        } else {
            match kana::str_to_hiragana(input) {
                Ok(hiragana) => {
                    println!("{}", hiragana);
                },
                Err(why) => match_error(why)
            }
        }
    }
}

fn match_error(error: custom_error::Error) {
    match error.kind() {
        custom_error::Kind::ConnectionError => eprintln!("Error connecting to database: {}", error.message()),
        custom_error::Kind::RepositoryError => eprintln!("Error querying database: {}", error.message()),
        custom_error::Kind::KanaError => eprintln!("Error transforming kana: {}", error.message())
    }
}