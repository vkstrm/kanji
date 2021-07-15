use kanabake;

use crate::custom_error::Error;

pub type Katakana = String;
pub type Hiragana = String;

pub fn str_to_katakana(input: &str) -> Result<Katakana, Error> {
    let katakana = kanabake::to_katakana(input).map_err(|why| Error::new_kana_error(why.to_string()))?;
    Ok(katakana)
}

pub fn str_to_hiragana(input: &str) -> Result<Hiragana, Error> {
    let hiragana = kanabake::to_hiragana(input).map_err(|why| Error::new_kana_error(why.to_string()))?;
    Ok(hiragana)
}