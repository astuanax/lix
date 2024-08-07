use regex::Regex;

pub struct LixNumber;

impl LixNumber {
    pub fn new() -> Self {
        LixNumber
    }

    pub fn calculate(&self, text: &str) -> f64 {
        let sentence_count = self.count_sentences(text);
        let word_count = self.count_words(text);
        let long_word_count = self.count_long_words(text);

        if word_count == 0 {
            return 0.0;
        }

        // LIX formula
        let lix = (word_count as f64 / sentence_count as f64)
            + (100.0 * (long_word_count as f64 / word_count as f64));

        lix.clamp(0.0, 100.0)
    }

    fn count_sentences(&self, text: &str) -> usize {
        let re = Regex::new(r"[.!?]").unwrap();
        re.find_iter(text).count()
    }

    fn count_words(&self, text: &str) -> usize {
        let re = Regex::new(r"\w+").unwrap();
        re.find_iter(text).count()
    }

    fn count_long_words(&self, text: &str) -> usize {
        let re = Regex::new(r"\b\w{7,}\b").unwrap(); // Words with 7 or more characters
        re.find_iter(text).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swedish() {
        let lix = LixNumber::new();
        let text = "Det här är en testmening. Den är utformad för att kontrollera LIX-talet.";
        let score = lix.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_norwegian() {
        let lix = LixNumber::new();
        let text = "Dette er en testsetning. Den er laget for å sjekke LIX-nummeret.";
        let score = lix.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_danish() {
        let lix = LixNumber::new();
        let text = "Dette er en test sætning. Den er designet til at kontrollere LIX-nummeret.";
        let score = lix.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_finnish() {
        let lix = LixNumber::new();
        let text = "Tämä on testilause. Se on suunniteltu tarkistamaan LIX-luku.";
        let score = lix.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_german() {
        let lix = LixNumber::new();
        let text = "Dies ist ein Testsatz. Er ist darauf ausgelegt, die LIX-Zahl zu überprüfen.";
        let score = lix.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_dutch() {
        let lix = LixNumber::new();
        let text = "Dit is een testzin. Het is ontworpen om het LIX-nummer te controleren.";
        let score = lix.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_english() {
        let lix = LixNumber::new();
        let text = "This is a test sentence. It is designed to check the LIX number.";
        let score = lix.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_french() {
        let lix = LixNumber::new();
        let text = "Ceci est une phrase de test. Elle est conçue pour vérifier le numéro LIX.";
        let score = lix.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_polish() {
        let lix = LixNumber::new();
        let text = "To jest zdanie testowe. Jest zaprojektowane, aby sprawdzić numer LIX.";
        let score = lix.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_bulgarian() {
        let lix = LixNumber::new();
        let text = "Това е тестово изречение. То е предназначено да провери LIX номера.";
        let score = lix.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_edge_cases() {
        let lix = LixNumber::new();
        let text = "";
        let score = lix.calculate(text);
        assert_eq!(score, 0.0);

        let text = "A.";
        let score = lix.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }
}
