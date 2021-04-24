use std::env;
use rusqlite::{Connection};

use crate::kana;
use crate::custom_error::Error;
use crate::custom_error;

pub struct RowDto {
    pub character: String,
    pub kunyomi: String,
    pub onyomi: String,
    pub meaning: String
}

pub fn query_character(value: &String) -> Result<Vec<RowDto>, Error> {
    query_map(&sql::character(), &sql::format_kanji(value))
}

pub fn query_meaning(value: &String) -> Result<Vec<RowDto>, Error> { 
    query_map(&sql::meaning(), &sql::format_like(value))
}

pub fn query_onyomi(kana: &kana::Katakana) -> Result<Vec<RowDto>, Error> {
    query_map(&sql::onyomi(), &sql::format_like(&kana.word))
}

pub fn query_kunyomi(kana: &kana::Hiragana) -> Result<Vec<RowDto>, Error> {
    query_map(&sql::kunyomi(), &sql::format_like(&kana.word))
}

fn database_path() -> Result<String, Error> {
    env::var("DB_PATH").map_err(|why| Error::new(custom_error::Kind::ConnectionError, why.to_string()))
}

fn get_connection() -> Result<Connection, Error>  {
    let path = database_path()?;
    Connection::open(&path).map_err(|why| Error::new(custom_error::Kind::ConnectionError, why.to_string()))
}

fn query_map(sql: &String, format: &String) -> Result<Vec<RowDto>, Error> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(sql.as_str()).map_err(|why| Error::new(custom_error::Kind::RepositoryError, why.to_string()))?;

    let iter = stmt.query_map_named(&[(sql::PARAM, &format)], |row| {
        Ok(RowDto {
            character: row.get(0)?,
            kunyomi: row.get(1)?,
            onyomi: row.get(2)?,
            meaning: row.get(3)?
        })
    }).map_err(|why| Error::new(custom_error::Kind::RepositoryError, why.to_string()))?;

    Ok(iter.map(|i| i.unwrap()).collect())
}

mod sql {
    pub const PARAM: &str = ":param";

    const ROW_BASE: &str = "SELECT kanji, kunyomi, onyomi, meaning FROM kanjidict";

    pub fn meaning() -> String {
        return String::from(ROW_BASE) + " WHERE LIKE(:param, meaning)";
    }

    pub fn onyomi() -> String {
        return String::from(ROW_BASE) + " WHERE LIKE(:param, onyomi)";
    }

    pub fn kunyomi() -> String {
        return String::from(ROW_BASE) + " WHERE LIKE(:param, kunyomi)";
    }

    pub fn character() -> String {
        return String::from(ROW_BASE) + " WHERE kanji = :param";
    }

    pub fn format_like(value: &String) -> String {
        format!("%{}%", value)
    }

    pub fn format_kanji(value: &String) -> String {
        format!("{}", value)
    }
}