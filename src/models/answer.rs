pub trait ValidateAnswer {
    fn validate_answer(&self, answer: &str) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnswerResult {
    Good,
    Wrong,
}
