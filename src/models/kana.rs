use strum::EnumIter;

#[derive(Debug, Clone)]
pub enum KanaRepresentation {
    Hiragana,
    Katakana,
}

impl std::fmt::Display for KanaRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KanaRepresentation::Hiragana => write!(f, "hiragana"),
            KanaRepresentation::Katakana => write!(f, "katakana"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum Kana {
    N,
    // ∅
    A,
    I,
    U,
    E,
    O,
    // K
    Ka,
    Ki,
    Ku,
    Ke,
    Ko,
    // S
    Sa,
    Shi,
    Su,
    Se,
    So,
    // T
    Ta,
    Chi,
    Tsu,
    Te,
    To,
    // N
    Na,
    Ni,
    Nu,
    Ne,
    No,
    // H
    Ha,
    Hi,
    Fu,
    He,
    Ho,
    // M
    Ma,
    Mi,
    Mu,
    Me,
    Mo,
    // Y
    Ya,
    Yu,
    Yo,
    // R
    Ra,
    Ri,
    Ru,
    Re,
    Ro,
    // W
    Wa,
    Wo,
    // --- diacritics
    // G
    Ga,
    Gi,
    Gu,
    Ge,
    Go,
    // Z
    Za,
    Ji,
    Zu,
    Ze,
    Zo,
    // D
    Da,
    Dji,
    Dzu,
    De,
    Do,
    // B
    Ba,
    Bi,
    Bu,
    Be,
    Bo,
    // P
    Pa,
    Pi,
    Pu,
    Pe,
    Po,
}

impl Kana {
    pub fn to_hiragana(&self) -> &str {
        match self {
            Self::N => "ん",
            Self::A => "あ",
            Self::I => "い",
            Self::U => "う",
            Self::E => "え",
            Self::O => "お",
            Self::Ka => "か",
            Self::Ki => "き",
            Self::Ku => "く",
            Self::Ke => "け",
            Self::Ko => "こ",
            Self::Sa => "さ",
            Self::Shi => "し",
            Self::Su => "す",
            Self::Se => "せ",
            Self::So => "そ",
            Self::Ta => "た",
            Self::Chi => "ち",
            Self::Tsu => "つ",
            Self::Te => "て",
            Self::To => "と",
            Self::Na => "な",
            Self::Ni => "に",
            Self::Nu => "ぬ",
            Self::Ne => "ね",
            Self::No => "の",
            Self::Ha => "は",
            Self::Hi => "ひ",
            Self::Fu => "ふ",
            Self::He => "へ",
            Self::Ho => "ほ",
            Self::Ma => "ま",
            Self::Mi => "み",
            Self::Mu => "む",
            Self::Me => "め",
            Self::Mo => "も",
            Self::Ya => "や",
            Self::Yu => "ゆ",
            Self::Yo => "よ",
            Self::Ra => "ら",
            Self::Ri => "り",
            Self::Ru => "る",
            Self::Re => "れ",
            Self::Ro => "ろ",
            Self::Wa => "わ",
            Self::Wo => "を",
            // ---
            Self::Ga => "が",
            Self::Gi => "ぎ",
            Self::Gu => "ぐ",
            Self::Ge => "げ",
            Self::Go => "ご",
            Self::Za => "ざ",
            Self::Ji => "じ",
            Self::Zu => "ず",
            Self::Ze => "ぜ",
            Self::Zo => "ぞ",
            Self::Da => "だ",
            Self::Dji => "ぢ",
            Self::Dzu => "づ",
            Self::De => "で",
            Self::Do => "ど",
            Self::Ba => "ば",
            Self::Bi => "び",
            Self::Bu => "ぶ",
            Self::Be => "べ",
            Self::Bo => "ぼ",
            Self::Pa => "ぱ",
            Self::Pi => "ぴ",
            Self::Pu => "ぷ",
            Self::Pe => "ぺ",
            Self::Po => "ぽ",
        }
    }

    pub fn to_katakana(&self) -> &str {
        match self {
            Self::N => "ン",
            Self::A => "ア",
            Self::I => "イ",
            Self::U => "ウ",
            Self::E => "エ",
            Self::O => "オ",
            Self::Ka => "カ",
            Self::Ki => "キ",
            Self::Ku => "ク",
            Self::Ke => "ケ",
            Self::Ko => "コ",
            Self::Sa => "サ",
            Self::Shi => "シ",
            Self::Su => "ス",
            Self::Se => "セ",
            Self::So => "ソ",
            Self::Ta => "タ",
            Self::Chi => "チ",
            Self::Tsu => "ツ",
            Self::Te => "テ",
            Self::To => "ト",
            Self::Na => "ナ",
            Self::Ni => "ニ",
            Self::Nu => "ヌ",
            Self::Ne => "ネ",
            Self::No => "ノ",
            Self::Ha => "ハ",
            Self::Hi => "ヒ",
            Self::Fu => "フ",
            Self::He => "ヘ",
            Self::Ho => "ホ",
            Self::Ma => "マ",
            Self::Mi => "ミ",
            Self::Mu => "ム",
            Self::Me => "メ",
            Self::Mo => "モ",
            Self::Ya => "ヤ",
            Self::Yu => "ユ",
            Self::Yo => "ヨ",
            Self::Ra => "ラ",
            Self::Ri => "リ",
            Self::Ru => "ル",
            Self::Re => "レ",
            Self::Ro => "ロ",
            Self::Wa => "ワ",
            Self::Wo => "ヲ",
            // ---
            Self::Ga => "ガ",
            Self::Gi => "ギ",
            Self::Gu => "グ",
            Self::Ge => "ゲ",
            Self::Go => "ゴ",
            Self::Za => "ザ",
            Self::Ji => "ジ",
            Self::Zu => "ズ",
            Self::Ze => "ゼ",
            Self::Zo => "ゾ",
            Self::Da => "ダ",
            Self::Dji => "ヂ",
            Self::Dzu => "ヅ",
            Self::De => "デ",
            Self::Do => "ド",
            Self::Ba => "バ",
            Self::Bi => "ビ",
            Self::Bu => "ブ",
            Self::Be => "ベ",
            Self::Bo => "ボ",
            Self::Pa => "パ",
            Self::Pi => "ピ",
            Self::Pu => "プ",
            Self::Pe => "ペ",
            Self::Po => "ポ",
        }
    }
}

impl std::fmt::Display for Kana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Kana::N => "n",
            Kana::A => "a",
            Kana::I => "i",
            Kana::U => "u",
            Kana::E => "e",
            Kana::O => "o",
            Kana::Ka => "ka",
            Kana::Ki => "ki",
            Kana::Ku => "ku",
            Kana::Ke => "ke",
            Kana::Ko => "ko",
            Kana::Sa => "sa",
            Kana::Shi => "shi",
            Kana::Su => "su",
            Kana::Se => "se",
            Kana::So => "so",
            Kana::Ta => "ta",
            Kana::Chi => "tchi",
            Kana::Tsu => "tsu",
            Kana::Te => "te",
            Kana::To => "to",
            Kana::Na => "na",
            Kana::Ni => "ni",
            Kana::Nu => "nu",
            Kana::Ne => "ne",
            Kana::No => "no",
            Kana::Ha => "ha",
            Kana::Hi => "hi",
            Kana::Fu => "fu",
            Kana::He => "he",
            Kana::Ho => "ho",
            Kana::Ma => "ma",
            Kana::Mi => "mi",
            Kana::Mu => "mu",
            Kana::Me => "me",
            Kana::Mo => "mo",
            Kana::Ya => "ya",
            Kana::Yu => "yu",
            Kana::Yo => "yo",
            Kana::Ra => "ra",
            Kana::Ri => "ri",
            Kana::Ru => "ru",
            Kana::Re => "re",
            Kana::Ro => "ro",
            Kana::Wa => "wa",
            Kana::Wo => "wo",
            Kana::Ga => "ga",
            Kana::Gi => "gi",
            Kana::Gu => "gu",
            Kana::Ge => "ge",
            Kana::Go => "go",
            Kana::Za => "za",
            Kana::Ji => "ji",
            Kana::Zu => "zu",
            Kana::Ze => "ze",
            Kana::Zo => "zo",
            Kana::Da => "da",
            Kana::Dji => "dji",
            Kana::Dzu => "dzu",
            Kana::De => "de",
            Kana::Do => "do",
            Kana::Ba => "ba",
            Kana::Bi => "bi",
            Kana::Bu => "bu",
            Kana::Be => "be",
            Kana::Bo => "bo",
            Kana::Pa => "pa",
            Kana::Pi => "pi",
            Kana::Pu => "pu",
            Kana::Pe => "pe",
            Kana::Po => "po",
        };
        write!(f, "{}", value)
    }
}

impl super::answer::ValidateAnswer for Kana {
    fn validate_answer(&self, answer: &str) -> bool {
        match (self, answer) {
            // allow some flexibility
            (Self::Chi, "chi" | "tchi") => true,
            (Self::Dji, "dji" | "ji") => true,
            // same as Display
            (kana, answer) => kana.to_string() == answer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::answer::ValidateAnswer;

    #[test]
    fn validate_multiple_answers() {
        assert!(Kana::A.validate_answer("a"));
        assert!(Kana::Chi.validate_answer("chi") && Kana::Chi.validate_answer("tchi"));
        assert!(Kana::Dji.validate_answer("dji") && Kana::Dji.validate_answer("ji"));
    }
}
