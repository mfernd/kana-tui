use super::kana::{Kana, KanaDiacritic};

pub trait Katakana {
    fn to_katakana(&self) -> &str;
}

impl Katakana for Kana {
    fn to_katakana(&self) -> &str {
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
        }
    }
}

impl Katakana for KanaDiacritic {
    fn to_katakana(&self) -> &str {
        match self {
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
