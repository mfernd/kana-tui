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

impl super::ValidateGuess for Kana {
    fn validate_guess(&self, guess: &str) -> bool {
        matches!(
            (self, guess),
            (Self::N, "n")
                | (Self::A, "a")
                | (Self::I, "i")
                | (Self::U, "u")
                | (Self::E, "e")
                | (Self::O, "o")
                | (Self::Ka, "ka")
                | (Self::Ki, "ki")
                | (Self::Ku, "ku")
                | (Self::Ke, "ke")
                | (Self::Ko, "ko")
                | (Self::Sa, "sa")
                | (Self::Shi, "shi")
                | (Self::Su, "su")
                | (Self::Se, "se")
                | (Self::So, "so")
                | (Self::Ta, "ta")
                | (Self::Chi, "chi" | "tchi")
                | (Self::Tsu, "tsu")
                | (Self::Te, "te")
                | (Self::To, "to")
                | (Self::Na, "na")
                | (Self::Ni, "ni")
                | (Self::Nu, "nu")
                | (Self::Ne, "ne")
                | (Self::No, "no")
                | (Self::Ha, "ha")
                | (Self::Hi, "hi")
                | (Self::Fu, "fu")
                | (Self::He, "he")
                | (Self::Ho, "ho")
                | (Self::Ma, "ma")
                | (Self::Mi, "mi")
                | (Self::Mu, "mu")
                | (Self::Me, "me")
                | (Self::Mo, "mo")
                | (Self::Ya, "ya")
                | (Self::Yu, "yu")
                | (Self::Yo, "yo")
                | (Self::Ra, "ra")
                | (Self::Ri, "ri")
                | (Self::Ru, "ru")
                | (Self::Re, "re")
                | (Self::Ro, "ro")
                | (Self::Wa, "wa")
                | (Self::Wo, "wo")
                | (Self::Ga, "ga")
                | (Self::Gi, "gi")
                | (Self::Gu, "gu")
                | (Self::Ge, "ge")
                | (Self::Go, "go")
                | (Self::Za, "za")
                | (Self::Ji, "ji")
                | (Self::Zu, "zu")
                | (Self::Ze, "ze")
                | (Self::Zo, "zo")
                | (Self::Da, "da")
                | (Self::Dji, "dji" | "ji")
                | (Self::Dzu, "dzu")
                | (Self::De, "de")
                | (Self::Do, "do")
                | (Self::Ba, "ba")
                | (Self::Bi, "bi")
                | (Self::Bu, "bu")
                | (Self::Be, "be")
                | (Self::Bo, "bo")
                | (Self::Pa, "pa")
                | (Self::Pi, "pi")
                | (Self::Pu, "pu")
                | (Self::Pe, "pe")
                | (Self::Po, "po")
        )
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
