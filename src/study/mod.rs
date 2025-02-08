use kana::Kana;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

pub mod kana;

pub trait ValidateGuess {
    fn validate_guess(&self, guess: &str) -> bool;
}

pub fn create_study_plan() -> Vec<Kana> {
    let mut kanas: Vec<Kana> = Kana::iter().collect();
    kanas.shuffle(&mut rand::rng());

    kanas
}
