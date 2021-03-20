use clap::{ArgMatches};

mod dictionary;
mod kana;
mod printer;

pub struct Row {
    pub character: String,
    pub kunyomi: Vec<String>,
    pub onyomi: Vec<String>,
    pub meaning: Vec<String>
}

pub fn handle_arguments(matches: &ArgMatches) {
    match matches.value_of("input") {
        Some(input) => {
            match matches.is_present("meaning") {
                true => {
                    let rows: Vec<Row>;
                    if matches.is_present("substring") {
                        rows = dictionary::by_meaning_substring(input);
                    } else {
                        rows = dictionary::by_meaning(input);
                    }
                    printer::print(&rows);
                }, _ => {}
            }
            match matches.is_present("onyomi") {
                true => {
                    let rows = dictionary::by_onyomi(input);
                    printer::print(&rows);
                }, _ => {}
            }
            match matches.is_present("kunyomi") {
                true => {
                    let rows = dictionary::by_kunyomi(input);
                    printer::print(&rows);
                }, _ => {}
            }
            match matches.is_present("kanji") {
                true => {
                    let rows = dictionary::by_character(input);
                    printer::print(&rows);
                }, _ => {}
            }
        
            // TODO query on both on and kun and get distinct result back
        }
        None => {}
    }
}