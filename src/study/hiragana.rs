use super::kana::{Kana, KanaDiacritic};

pub trait Hiragana {
    fn to_hiragana(&self) -> &str;
}

impl Hiragana for Kana {
    fn to_hiragana(&self) -> &str {
        match self {
            Kana::N => "ん",
            Kana::A => "あ",
            Kana::I => "い",
            Kana::U => "う",
            Kana::E => "え",
            Kana::O => "お",
            Kana::KA => "か",
            Kana::KI => "き",
            Kana::KU => "く",
            Kana::KE => "け",
            Kana::KO => "こ",
            Kana::SA => "さ",
            Kana::SHI => "し",
            Kana::SU => "す",
            Kana::SE => "せ",
            Kana::SO => "そ",
            Kana::TA => "た",
            Kana::CHI => "ち",
            Kana::TSU => "つ",
            Kana::TE => "て",
            Kana::TO => "と",
            Kana::NA => "な",
            Kana::NI => "に",
            Kana::NU => "ぬ",
            Kana::NE => "ね",
            Kana::NO => "の",
            Kana::HA => "は",
            Kana::HI => "ひ",
            Kana::FU => "ふ",
            Kana::HE => "へ",
            Kana::HO => "ほ",
            Kana::MA => "ま",
            Kana::MI => "み",
            Kana::MU => "む",
            Kana::ME => "め",
            Kana::MO => "も",
            Kana::YA => "や",
            Kana::YU => "ゆ",
            Kana::YO => "よ",
            Kana::RA => "ら",
            Kana::RI => "り",
            Kana::RU => "る",
            Kana::RE => "れ",
            Kana::RO => "ろ",
            Kana::WA => "わ",
            Kana::WO => "を",
        }
    }
}

impl Hiragana for KanaDiacritic {
    fn to_hiragana(&self) -> &str {
        match self {
            KanaDiacritic::GA => "が",
            KanaDiacritic::GI => "ぎ",
            KanaDiacritic::GU => "ぐ",
            KanaDiacritic::GE => "げ",
            KanaDiacritic::GO => "ご",
            KanaDiacritic::ZA => "ざ",
            KanaDiacritic::JI => "じ",
            KanaDiacritic::ZU => "ず",
            KanaDiacritic::ZE => "ぜ",
            KanaDiacritic::ZO => "ぞ",
            KanaDiacritic::DA => "だ",
            KanaDiacritic::DJI => "ぢ",
            KanaDiacritic::DZU => "づ",
            KanaDiacritic::DE => "で",
            KanaDiacritic::DO => "ど",
            KanaDiacritic::BA => "ば",
            KanaDiacritic::BI => "び",
            KanaDiacritic::BU => "ぶ",
            KanaDiacritic::BE => "べ",
            KanaDiacritic::BO => "ぼ",
            KanaDiacritic::PA => "ぱ",
            KanaDiacritic::PI => "ぴ",
            KanaDiacritic::PU => "ぷ",
            KanaDiacritic::PE => "ぺ",
            KanaDiacritic::PO => "ぽ",
        }
    }
}
