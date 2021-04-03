use std::collections::HashMap;

pub enum KanaType {
    Hiragana,
    Katakana
}

pub struct Sets {
    pub kana_dict: HashMap<&'static str, &'static str>,
}

impl Sets {
    pub fn new(set_type: KanaType) -> Sets {
        Sets {
            kana_dict: match set_type {
                KanaType::Hiragana => hiragana(),
                KanaType::Katakana => katakana(),
            }
        }
    }

    pub fn get(&mut self, key: &String) -> &str {
        let s = self.kana_dict.get(&key.as_str()).expect("No such token in map");
        s
    }
}

pub fn hiragana() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("A", "あ");
    map.insert("I","い");
    map.insert("U","う");
    map.insert("E","え");
    map.insert("O","お");
    map.insert("KA","か");
    map.insert("KI","き");
    map.insert("KYA","きゃ");
    map.insert("KYU","きゅ");
    map.insert("KYO","きょ");
    map.insert("KU","く");
    map.insert("KE","け");
    map.insert("KO","こ");
    map.insert("GA","が");
    map.insert("GI", "ぎ");
    map.insert("GYA","ぎゃ");
    map.insert("GYU","ぎゅ");
    map.insert("GYO","ぎょ");
    map.insert("GU", "ぐ");
    map.insert("GE","げ");
    map.insert("GO","ご");
    map.insert("SA","さ");
    map.insert("SHI","し");
    map.insert("SHA","しゃ");
    map.insert("SHU","しゅ");
    map.insert("SHO","しょ");
    map.insert("SU","す");
    map.insert("SE","せ");
    map.insert("SO","そ");
    map.insert("ZA","ざ");
    map.insert("JI","じ");
    map.insert("JA","じゃ");
    map.insert("JU","じゅ");
    map.insert("JO","じょ");
    map.insert("JYA","じゃ");
    map.insert("JYU","じゅ");
    map.insert("JYO","じょ");
    map.insert("ZU","ず");
    map.insert("ZE","ぜ");
    map.insert("ZO","ぞ");
    map.insert("TA","た");
    map.insert("CHI","ち");
    map.insert("CHA","ちゃ");
    map.insert("CHU","ちゅ");
    map.insert("CHO","ちょ");
    map.insert("TSU","つ");
    map.insert("TE","て");
    map.insert("TO","と");
    map.insert("DA","だ");
    map.insert("DI","ぢ");
    map.insert("DYA","ぢゃ");
    map.insert("DYU","ぢゅ");
    map.insert("DYO","ぢょ");
    map.insert("DU","づ");
    map.insert("DZU","づ");
    map.insert("DE","で");
    map.insert("DO","ど");
    map.insert("NA","な");
    map.insert("NI","に");
    map.insert("NYA","にゃ");
    map.insert("NYU","にゅ");
    map.insert("NYO","にょ");
    map.insert("NU","ぬ");
    map.insert("NE","ね");
    map.insert("NO","の");
    map.insert("HA","は");
    map.insert("HI","ひ");
    map.insert("HYA","ひゃ");
    map.insert("HYU","ひゅ");
    map.insert("HYO","ひょ");
    map.insert("FU","ふ");
    map.insert("HE","へ");
    map.insert("HO","ほ"); 
    map.insert("BA","ば");
    map.insert("BI","び");
    map.insert("BYA","びゃ");
    map.insert("BYU","びゅ");
    map.insert("BYO","びょ");
    map.insert("BU","ぶ");
    map.insert("BE","べ");  
    map.insert("BO","ぼ"); 
    map.insert("PA","ぱ");
    map.insert("PI","ぴ");
    map.insert("PYA","ぴゃ");
    map.insert("PYU","ぴゅ");
    map.insert("PYO","ぴょ");
    map.insert("PU","ぷ");
    map.insert("PE","ぺ");
    map.insert("PO","ぽ");
    map.insert("MA","ま");
    map.insert("MI","み");
    map.insert("MYA","みゃ");
    map.insert("MYU","みゅ");
    map.insert("MYO","みょ");
    map.insert("MU","む");
    map.insert("ME","め");
    map.insert("MO","も");
    map.insert("YA","や");
    map.insert("YU","ゆ");
    map.insert("YO","よ");
    map.insert("RA","ら");
    map.insert("RI","り");
    map.insert("RYA","りゃ");
    map.insert("RYU","りゅ");
    map.insert("RYO","りょ"); 
    map.insert("RU","る");
    map.insert("RE","れ");
    map.insert("RO","ろ");
    map.insert("WA","わ");
    map.insert("WO","を");
    map.insert("N","ん");
    map.insert("LTSU","っ");
    map
}

pub fn katakana() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("A", "ア");
    map.insert("I","イ");
    map.insert("U","ウ");
    map.insert("E","エ");
    map.insert("O","オ");
    map.insert("KA","カ");
    map.insert("KI","キ");
    map.insert("KYA","キャ");
    map.insert("KYU","キュ");
    map.insert("KYO","キョ");
    map.insert("KU","ク");
    map.insert("KE","ケ");
    map.insert("KO","コ");
    map.insert("GA","ガ");
    map.insert("GI", "ギ");
    map.insert("GYA","ギャ");
    map.insert("GYU","ギュ");
    map.insert("GYO","ギョ");
    map.insert("GU", "グ");
    map.insert("GE","ゲ");
    map.insert("GO","ゴ");
    map.insert("SA","サ");
    map.insert("SHI","シ");
    map.insert("SHA","シャ");
    map.insert("SHU","シュ");
    map.insert("SHO","ショ");
    map.insert("SU","ス");
    map.insert("SE","セ");
    map.insert("SO","ソ");
    map.insert("ZA","ザ");
    map.insert("JI","ジ");
    map.insert("JA","ジャ");
    map.insert("JU","ジュ");
    map.insert("JO","ジョ");
    map.insert("JYA","ジャ");
    map.insert("JYU","ジュ");
    map.insert("JYO","ジョ");
    map.insert("ZU","ズ");
    map.insert("ZE","ゼ");
    map.insert("ZO","ゾ");
    map.insert("TA","タ");
    map.insert("CHI","チ");
    map.insert("CHA","チャ");
    map.insert("CHU","チュ");
    map.insert("CHO","チョ");
    map.insert("TSU","ツ");
    map.insert("TE","テ");
    map.insert("TO","ト");
    map.insert("DA","ダ");
    map.insert("DI","ヂ");
    map.insert("DYA","ヂャ");
    map.insert("DYU","ヂュ");
    map.insert("DYO","ヂョ");
    map.insert("DU","ヅ");
    map.insert("DZU","ヅ");
    map.insert("DE","デ");
    map.insert("DO","ド");
    map.insert("NA","ナ");
    map.insert("NI","ニ");
    map.insert("NYA","ニャ");
    map.insert("NYU","ニュ");
    map.insert("NYO","ニョ");
    map.insert("NU","ヌ");
    map.insert("NE","ネ");
    map.insert("NO","ノ");
    map.insert("HA","ハ");
    map.insert("HI","ヒ");
    map.insert("HYA","ヒャ");
    map.insert("HYU","ヒュ");
    map.insert("HYO","ヒョ");
    map.insert("FU","フ");
    map.insert("HE","ヘ");
    map.insert("HO","ホ"); 
    map.insert("BA","バ");
    map.insert("BI","ビ");
    map.insert("BYA","ビャ");
    map.insert("BYU","ビュ");
    map.insert("BYO","ビョ");
    map.insert("BU","ブ");
    map.insert("BE","ベ");  
    map.insert("BO","ボ"); 
    map.insert("PA","パ");
    map.insert("PI","ピ");
    map.insert("PYA","ピャ");
    map.insert("PYU","ピュ");
    map.insert("PYO","ピョ");
    map.insert("PU","プ");
    map.insert("PE","ペ");
    map.insert("PO","ポ");
    map.insert("MA","マ");
    map.insert("MI","ミ");
    map.insert("MYA","ミャ");
    map.insert("MYU","ミュ");
    map.insert("MYO","ミョ");
    map.insert("MU","ム");
    map.insert("ME","メ");
    map.insert("MO","モ");
    map.insert("YA","ヤ");
    map.insert("YU","ユ");
    map.insert("YO","ヨ");
    map.insert("RA","ラ");
    map.insert("RI","リ");
    map.insert("RYA","リャ");
    map.insert("RYU","リュ");
    map.insert("RYO","リョ"); 
    map.insert("RU","ル");
    map.insert("RE","レ");
    map.insert("RO","ロ");
    map.insert("WA","ワ");
    map.insert("WO","ヲ");
    map.insert("N","ン");
    map.insert("LTSU","ッ");
    map
}