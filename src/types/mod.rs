
#[derive(Debug)]
pub struct Row {
    pub character: String,
    pub kunyomi: String,
    pub onyomi: String,
    pub meaning: String
}

pub struct NanoriRow {
    pub id: i32,
    pub character: String,
    pub nanori: String
}