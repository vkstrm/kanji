use crate::kana;
use crate::custom_error;

mod repository;

type Row = super::Row;

pub fn by_character(input: &str) -> Result<Vec<Row>, custom_error::Error> {
    let dtos = repository::query_character(&String::from(input))?;
    Ok(collect_rows(dtos))
}

pub fn by_meaning(input: &str) -> Result<Vec<Row>, custom_error::Error> {
    let rows = meaning(input)?;
    Ok(filter_rows(rows, &String::from(input)))
}

pub fn by_meaning_substring(input: &str) -> Result<Vec<Row>, custom_error::Error> {
    Ok(meaning(input))?
}

pub fn by_onyomi(input: &str) -> Result<Vec<Row>, custom_error::Error> {
    let kana = kana::str_to_katakana(input);
    let dtos = repository::query_onyomi(&kana)?;
    Ok(collect_rows(dtos))
}

pub fn by_kunyomi(input: &str) -> Result<Vec<Row>, custom_error::Error> {
    let kana = kana::str_to_hiragana(input);
    let dtos = repository::query_kunyomi(&kana)?;
    Ok(collect_rows(dtos))
}

fn meaning(input: &str) -> Result<Vec<Row>, custom_error::Error> {
    let dtos = repository::query_meaning(&String::from(input))?;
    Ok(collect_rows(dtos))
}

fn filter_rows(rows: Vec<Row>, filter: &String) -> Vec<Row> {
    let mut filtered: Vec<Row> = Vec::new();
    for row in rows {
        for meaning in &row.meaning {
            if meaning.to_ascii_uppercase() == filter.to_ascii_uppercase() {
                filtered.push(row);
                break;
            }
        }
    }
    filtered
}

fn collect_rows(dtos: Vec<repository::RowDto>) -> Vec<Row> {
    dtos.into_iter().map(|r| from_dto(r)).collect()   
}

fn from_dto(dto: repository::RowDto) -> Row {
    Row {
        character: dto.character,
        onyomi: dto.onyomi.split("、").map(|s| String::from(s)).collect(),
        kunyomi: dto.kunyomi.split("、").map(|s| String::from(s)).collect(),
        meaning: dto.meaning.split(";").map(|s| String::from(s)).collect()
    }
}