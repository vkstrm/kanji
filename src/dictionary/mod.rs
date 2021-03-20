use crate::kana;

mod repository;

type Row = super::Row;

pub fn by_character(input: &str) -> Vec<Row> {
    match repository::query_character(&String::from(input)) {
        Ok(dtos) => {
            return collect_rows(dtos);
        },
        Err(e) => panic!(e)
    }
}

pub fn by_meaning(input: &str) -> Vec<Row> {
    let rows = meaning(input);
    return filter_rows(rows, &String::from(input));
}

pub fn by_meaning_substring(input: &str) -> Vec<Row> {
    return meaning(input);
}

pub fn by_onyomi(input: &str) -> Vec<Row> {
    let kana = kana::str_to_katakana(input);
    match repository::query_onyomi(&kana) {
        Ok(dtos) => {
            return collect_rows(dtos);
        },
        Err(e) => panic!(e)
    }
}

pub fn by_kunyomi(input: &str) -> Vec<Row> {
    let kana = kana::str_to_hiragana(input);
    match repository::query_kunyomi(&kana) {
        Ok(dtos) => {
            return collect_rows(dtos);
        },
        Err(e) => panic!(e)
    }
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

fn meaning(input: &str) -> Vec<Row> {
    match repository::query_meaning(&String::from(input)) {
        Ok(dtos) => {
            let rows: Vec<Row> = collect_rows(dtos);
            return rows;
        },
        Err(e) => panic!(e)
    }
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