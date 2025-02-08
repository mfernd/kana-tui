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
}

impl super::ValidateGuess for Kana {
    fn validate(&self, guess: &str) -> bool {
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
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum KanaDiacritic {
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

impl super::ValidateGuess for KanaDiacritic {
    fn validate(&self, guess: &str) -> bool {
        match (self, guess) {
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
