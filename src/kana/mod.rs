mod tokens;
mod sets;

pub struct Katakana {
    pub word: String
}

pub struct Hiragana {
    pub word: String
}

pub fn str_to_katakana(input: &str) -> Katakana {
    if !input.is_ascii() {
        return Katakana{ word:String::from(input) };
    }
    to_katakana(&String::from(input).to_ascii_uppercase())
}

pub fn str_to_hiragana(input: &str) -> Hiragana {
    if !input.is_ascii() {
        return Hiragana{ word:String::from(input) };
    }
    to_hiragana(&String::from(input).to_ascii_uppercase())
}

fn to_katakana(ascii_input: &String) -> Katakana {
    let mut set = sets::Sets::new(sets::KanaType::Katakana);
    Katakana {
        word: transform(ascii_input, &mut set)
    }
}

fn to_hiragana(ascii_input: &String) -> Hiragana {
    let mut set = sets::Sets::new(sets::KanaType::Hiragana);
    let word = transform(ascii_input, &mut set); 
    Hiragana {
        word: word
    }
}

fn transform(input: &String, set: &mut sets::Sets) -> String {
    let tokens = tokens::interpret_tokens(&input);
    let mut word = String::new();
    for token in tokens {
        word.push_str(set.get(&token));
    }
    word
}