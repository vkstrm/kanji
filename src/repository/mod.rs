use rusqlite::{Connection, Result};

use crate::types;
type MeaningRow = types::Row;

fn get_connection() -> Connection {
    let path = "/home/viktor/Kod/github/kanjium/data/kanjidb.sqlite";
    match Connection::open(&path) {
        Ok(conn) => return conn,
        Err(e) => panic!(e),
    }
}

pub fn query_character(kanji: &String) {}


pub fn query_kanji_meaning(meaning: &String) -> Result<Vec<MeaningRow>> {
    let conn = get_connection();
    println!("here");
    let mut stmt = conn.prepare("
        SELECT kanji, kunyomi, onyomi, meaning FROM kanjidict \
        WHERE LIKE(:param, meaning)"
    )?;

    println!("here");
    let mut iter = stmt.query_map_named(&[(":param", &format!("%{}%", meaning))], |row| {
        Ok(MeaningRow {
            character: row.get(0)?,
            kunyomi: row.get(1)?,
            onyomi: row.get(2)?,
            meaning: row.get(3)?
        })
    })?;
    println!("here");
    
    let mut vec: Vec<MeaningRow> = Vec::new();
    for i in iter {
        let m = i.unwrap();
        println!("{:?}", m);
        //vec.push(i.unwrap());
    }

    println!("here");

    Ok(vec)
}

pub fn query_onyomi(onyomi: &String) {}

pub fn query_kunyomi(kunyomi: &String) {}