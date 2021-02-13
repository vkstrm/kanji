use clap::{ArgMatches};

mod types;
mod repository;

pub fn interpret_commands(matches: &ArgMatches) {
    let input: &str;
    match matches.value_of("input") {
        Some(i) => input = i,
        None => {
            println!("Missing input");
            return
        }
    }

    match matches.occurrences_of("meaning") {
        1 => query::meaning(input),
        _ => {} 
    }

    match matches.occurrences_of("onyomi") {
        1 => query::onyomi(input),
        _ => {} 
    }

    match matches.occurrences_of("kunyomi") {
        1 => query::kunyomi(input),
        _ => {} 
    }

    match matches.occurrences_of("kanji") {
        1 => query::kanji(input),
        _ => {}
    }

    match matches.occurrences_of("verbose") {
        1 => println!("Something should happen I guess"),
        _ => {}
    }

    query::hiragana(input);
}

mod query {
    use crate::repository;

    pub fn meaning(input: &str) {
        match repository::query_kanji_meaning(&String::from(input)) {
            Ok(rows) => {
                for elem in rows {
                    println!("{}", elem.character)
                }
            },
            Err(e) => panic!(e)
        }
        
    }

    pub fn onyomi(input: &str) {}

    pub fn kunyomi(input: &str) {}

    pub fn hiragana(input: &str) {}

    pub fn kanji(input: &str){}
}