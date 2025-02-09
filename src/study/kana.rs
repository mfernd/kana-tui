use strum::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Kana {
    N,
    // ∅
    A,
    I,
    U,
    E,
    O,
    // K
    KA,
    KI,
    KU,
    KE,
    KO,
    // S
    SA,
    SHI,
    SU,
    SE,
    SO,
    // T
    TA,
    CHI,
    TSU,
    TE,
    TO,
    // N
    NA,
    NI,
    NU,
    NE,
    NO,
    // H
    HA,
    HI,
    FU,
    HE,
    HO,
    // M
    MA,
    MI,
    MU,
    ME,
    MO,
    // Y
    YA,
    YU,
    YO,
    // R
    RA,
    RI,
    RU,
    RE,
    RO,
    // W
    WA,
    WO,
    // --- diacritics
    // G
    GA,
    GI,
    GU,
    GE,
    GO,
    // Z
    ZA,
    JI,
    ZU,
    ZE,
    ZO,
    // D
    DA,
    DJI,
    DZU,
    DE,
    DO,
    // B
    BA,
    BI,
    BU,
    BE,
    BO,
    // P
    PA,
    PI,
    PU,
    PE,
    PO,
}

impl super::ValidateGuess for Kana {
    fn validate_guess(&self, guess: &str) -> bool {
        match (self, guess) {
            (Self::N, "n") => true,
            (Self::A, "a") => true,
            (Self::I, "i") => true,
            (Self::U, "u") => true,
            (Self::E, "e") => true,
            (Self::O, "o") => true,
            (Self::KA, "ka") => true,
            (Self::KI, "ki") => true,
            (Self::KU, "ku") => true,
            (Self::KE, "ke") => true,
            (Self::KO, "ko") => true,
            (Self::SA, "sa") => true,
            (Self::SHI, "shi") => true,
            (Self::SU, "su") => true,
            (Self::SE, "se") => true,
            (Self::SO, "so") => true,
            (Self::TA, "ta") => true,
            (Self::CHI, "chi" | "tchi") => true,
            (Self::TSU, "tsu") => true,
            (Self::TE, "te") => true,
            (Self::TO, "to") => true,
            (Self::NA, "na") => true,
            (Self::NI, "ni") => true,
            (Self::NU, "nu") => true,
            (Self::NE, "ne") => true,
            (Self::NO, "no") => true,
            (Self::HA, "ha") => true,
            (Self::HI, "hi") => true,
            (Self::FU, "fu") => true,
            (Self::HE, "he") => true,
            (Self::HO, "ho") => true,
            (Self::MA, "ma") => true,
            (Self::MI, "mi") => true,
            (Self::MU, "mu") => true,
            (Self::ME, "me") => true,
            (Self::MO, "mo") => true,
            (Self::YA, "ya") => true,
            (Self::YU, "yu") => true,
            (Self::YO, "yo") => true,
            (Self::RA, "ra") => true,
            (Self::RI, "ri") => true,
            (Self::RU, "ru") => true,
            (Self::RE, "re") => true,
            (Self::RO, "ro") => true,
            (Self::WA, "wa") => true,
            (Self::WO, "wo") => true,
            (Self::GA, "ga") => true,
            (Self::GI, "gi") => true,
            (Self::GU, "gu") => true,
            (Self::GE, "ge") => true,
            (Self::GO, "go") => true,
            (Self::ZA, "za") => true,
            (Self::JI, "ji") => true,
            (Self::ZU, "zu") => true,
            (Self::ZE, "ze") => true,
            (Self::ZO, "zo") => true,
            (Self::DA, "da") => true,
            (Self::DJI, "dji" | "ji") => true,
            (Self::DZU, "dzu") => true,
            (Self::DE, "de") => true,
            (Self::DO, "do") => true,
            (Self::BA, "ba") => true,
            (Self::BI, "bi") => true,
            (Self::BU, "bu") => true,
            (Self::BE, "be") => true,
            (Self::BO, "bo") => true,
            (Self::PA, "pa") => true,
            (Self::PI, "pi") => true,
            (Self::PU, "pu") => true,
            (Self::PE, "pe") => true,
            (Self::PO, "po") => true,
            _ => false,
        }
    }
}

#[allow(dead_code)]
impl Kana {
    pub fn to_hiragana(&self) -> &str {
        match self {
            Self::N => "ん",
            Self::A => "あ",
            Self::I => "い",
            Self::U => "う",
            Self::E => "え",
            Self::O => "お",
            Self::KA => "か",
            Self::KI => "き",
            Self::KU => "く",
            Self::KE => "け",
            Self::KO => "こ",
            Self::SA => "さ",
            Self::SHI => "し",
            Self::SU => "す",
            Self::SE => "せ",
            Self::SO => "そ",
            Self::TA => "た",
            Self::CHI => "ち",
            Self::TSU => "つ",
            Self::TE => "て",
            Self::TO => "と",
            Self::NA => "な",
            Self::NI => "に",
            Self::NU => "ぬ",
            Self::NE => "ね",
            Self::NO => "の",
            Self::HA => "は",
            Self::HI => "ひ",
            Self::FU => "ふ",
            Self::HE => "へ",
            Self::HO => "ほ",
            Self::MA => "ま",
            Self::MI => "み",
            Self::MU => "む",
            Self::ME => "め",
            Self::MO => "も",
            Self::YA => "や",
            Self::YU => "ゆ",
            Self::YO => "よ",
            Self::RA => "ら",
            Self::RI => "り",
            Self::RU => "る",
            Self::RE => "れ",
            Self::RO => "ろ",
            Self::WA => "わ",
            Self::WO => "を",
            Self::GA => "が",
            Self::GI => "ぎ",
            Self::GU => "ぐ",
            Self::GE => "げ",
            Self::GO => "ご",
            Self::ZA => "ざ",
            Self::JI => "じ",
            Self::ZU => "ず",
            Self::ZE => "ぜ",
            Self::ZO => "ぞ",
            Self::DA => "だ",
            Self::DJI => "ぢ",
            Self::DZU => "づ",
            Self::DE => "で",
            Self::DO => "ど",
            Self::BA => "ば",
            Self::BI => "び",
            Self::BU => "ぶ",
            Self::BE => "べ",
            Self::BO => "ぼ",
            Self::PA => "ぱ",
            Self::PI => "ぴ",
            Self::PU => "ぷ",
            Self::PE => "ぺ",
            Self::PO => "ぽ",
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
            Self::KA => "カ",
            Self::KI => "キ",
            Self::KU => "ク",
            Self::KE => "ケ",
            Self::KO => "コ",
            Self::SA => "サ",
            Self::SHI => "シ",
            Self::SU => "ス",
            Self::SE => "セ",
            Self::SO => "ソ",
            Self::TA => "タ",
            Self::CHI => "チ",
            Self::TSU => "ツ",
            Self::TE => "テ",
            Self::TO => "ト",
            Self::NA => "ナ",
            Self::NI => "ニ",
            Self::NU => "ヌ",
            Self::NE => "ネ",
            Self::NO => "ノ",
            Self::HA => "ハ",
            Self::HI => "ヒ",
            Self::FU => "フ",
            Self::HE => "ヘ",
            Self::HO => "ホ",
            Self::MA => "マ",
            Self::MI => "ミ",
            Self::MU => "ム",
            Self::ME => "メ",
            Self::MO => "モ",
            Self::YA => "ヤ",
            Self::YU => "ユ",
            Self::YO => "ヨ",
            Self::RA => "ラ",
            Self::RI => "リ",
            Self::RU => "ル",
            Self::RE => "レ",
            Self::RO => "ロ",
            Self::WA => "ワ",
            Self::WO => "ヲ",
            Self::GA => "ガ",
            Self::GI => "ギ",
            Self::GU => "グ",
            Self::GE => "ゲ",
            Self::GO => "ゴ",
            Self::ZA => "ザ",
            Self::JI => "ジ",
            Self::ZU => "ズ",
            Self::ZE => "ゼ",
            Self::ZO => "ゾ",
            Self::DA => "ダ",
            Self::DJI => "ヂ",
            Self::DZU => "ヅ",
            Self::DE => "デ",
            Self::DO => "ド",
            Self::BA => "バ",
            Self::BI => "ビ",
            Self::BU => "ブ",
            Self::BE => "ベ",
            Self::BO => "ボ",
            Self::PA => "パ",
            Self::PI => "ピ",
            Self::PU => "プ",
            Self::PE => "ペ",
            Self::PO => "ポ",
        }
    }
}
