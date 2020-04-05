use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("big.txt").expect("File big.txt not found");

    let words = split_into_words(contents.as_str());

    let frequency = Frequency::new(words);

    println!("{}", frequency.num_words);
    println!("{}", frequency.probability("the"));
}

struct Frequency {
    map: HashMap<String, i32>,
    num_words: i32,
}

impl Frequency {
    pub fn new(words: Vec<&str>) -> Self {
        let mut frequency = HashMap::new();

        for &word in &words {
            let lower = word.to_lowercase();
            let counter = frequency.entry(lower).or_insert(0);
            *counter += 1;
        }

        Frequency {
            map: frequency,
            num_words: words.len() as i32,
        }
    }

    pub fn probability(&self, word: &str) -> f64 {
        if let Some(occurances) = self.map.get(word) {
            (*occurances as f64) / (self.num_words as f64)
        } else {
            0.0
        }
    }
}

fn split_into_words(contents: &str) -> Vec<&str> {
    let re = Regex::new(r"\w+").unwrap();
    re.find_iter(contents).map(|word| word.as_str()).collect()
}

mod tests {
    #[allow(unused_imports)]
    use super::Frequency;

    #[test]
    fn frequency() {
        let words = vec!["the", "the", "the", "ploy"];
        let frequency = Frequency::new(words);

        assert_eq!(frequency.map.get("the"), Some(&3));
        assert_eq!(frequency.map.get("ploy"), Some(&1));
        assert_eq!(frequency.map.get("dada"), None);
    }

    #[test]
    fn probability() {
        let words = vec!["the", "the", "the", "ploy"];
        let frequency = Frequency::new(words);

        assert_eq!(frequency.probability("the"), 0.75);
        assert_eq!(frequency.probability("ploy"), 0.25);
        assert_eq!(frequency.probability("dada"), 0.0);
    }
}
