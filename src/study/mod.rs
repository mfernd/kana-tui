use kana::Kana;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

pub mod hiragana;
pub mod kana;
pub mod katakana;

pub trait ValidateGuess {
    fn validate(&self, guess: &str) -> bool;
}

pub fn create_full_study_plan() -> Vec<Kana> {
    let mut all_kana: Vec<Kana> = Kana::iter().collect();
    all_kana.shuffle(&mut rand::rng());

    all_kana
}
