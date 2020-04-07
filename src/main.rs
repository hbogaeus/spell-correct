use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let contents = fs::read_to_string("big.txt").expect("File big.txt not found");

    let default = String::from("somthing");

    let word = env::args().skip(1).next().unwrap_or(default);

    let known_words = collect_unique_words(contents.as_str());

    let frequency = Frequency::new(&known_words);

    let candidates = candiates(&word, &known_words);

    let most_likely = find_most_likely(&candidates, &frequency);

    println!("{}", most_likely)
}

fn find_most_likely<'a>(candidates: &'a Vec<&str>, freq: &Frequency) -> &'a str {
    candidates
        .iter()
        .fold((0.0, ""), |(value, string), item| {
            let frequency = freq.probability(item);
            if frequency > value {
                (frequency, item)
            } else {
                (value, string)
            }
        })
        .1
}

fn candiates<'a>(word: &'a str, known_words: &'a HashSet<&str>) -> Vec<&'a str> {
    if let Some(canidates) = known(&vec![word.to_string()], known_words) {
        println!("Match!");
        canidates
    } else if let Some(candidates) = known(&edits1(&word), &known_words) {
        println!("Edit 1!");
        candidates
    } else if let Some(candidates) = known(&edits2(&word), &known_words) {
        println!("Edit 2!");
        candidates
    } else {
        println!("No match...");
        vec![&word]
    }
}

fn known<'a>(variants: &Vec<String>, words: &'a HashSet<&str>) -> Option<Vec<&'a str>> {
    let known_words: Vec<&str> = variants
        .iter()
        .map(|word| word.as_str())
        .filter_map(|variant| words.get(variant))
        .map(|word| *word)
        .collect();

    if known_words.is_empty() {
        None
    } else {
        Some(known_words)
    }
}

fn edits2(word: &str) -> Vec<String> {
    let initial_set = edits1(word);
    let mut final_set = HashSet::new();

    for variant in initial_set.iter() {
        for result in edits1(variant).iter() {
            final_set.insert(result.clone());
        }
    }

    final_set.into_iter().collect()
}

fn edits1(word: &str) -> Vec<String> {
    let mut set = HashSet::new();

    deletes(word, &mut set);
    transposes(word, &mut set);
    replaces(word, &mut set);
    inserts(word, &mut set);

    set.into_iter().collect()
}

fn deletes(word: &str, set: &mut HashSet<String>) {
    for pos in 0..word.len() {
        let mut new = String::new();
        new.push_str(&word[..pos]);
        new.push_str(&word[pos + 1..]);
        set.insert(new);
    }
}

fn inserts(word: &str, set: &mut HashSet<String>) {
    for pos in 0..word.len() + 1 {
        for letter in ALPHABET.chars() {
            let mut new = String::new();
            new.push_str(&word[..pos]);
            new.push(letter);
            new.push_str(&word[pos..]);
            set.insert(new);
        }
    }
}

fn replaces(word: &str, set: &mut HashSet<String>) {
    for pos in 0..word.len() {
        for letter in ALPHABET.chars() {
            let mut new = String::new();
            new.push_str(&word[..pos]);
            new.push(letter);
            new.push_str(&word[pos + 1..]);
            set.insert(new);
        }
    }
}

fn transposes(word: &str, set: &mut HashSet<String>) {
    for pos in 0..word.len() - 1 {
        let mut new = String::new();

        new.push_str(&word[..pos]);

        let current = &word[pos..pos + 1];
        let next = &word[pos + 1..pos + 2];

        new.push_str(next);
        new.push_str(current);

        new.push_str(&word[pos + 2..]);

        set.insert(new);
    }
}

struct Frequency {
    map: HashMap<String, i32>,
    num_words: i32,
}

impl Frequency {
    pub fn new(words: &HashSet<&str>) -> Self {
        let mut frequency = HashMap::new();

        for &word in words {
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

fn collect_unique_words(contents: &str) -> HashSet<&str> {
    let re = Regex::new(r"\w+").unwrap();
    re.find_iter(contents).map(|word| word.as_str()).collect()
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

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

    #[test]
    fn deletes() {
        let mut set = HashSet::new();
        let word = "abc";

        super::deletes(word, &mut set);

        assert_eq!(set.len(), 3);
        assert!(set.contains("bc"));
        assert!(set.contains("ac"));
        assert!(set.contains("ab"));
    }

    #[test]
    fn inserts() {
        let mut set = HashSet::new();
        let word = "abc";

        super::inserts(word, &mut set);

        assert_eq!(set.len(), ((word.len() + 1) * 26) - word.len());
    }

    #[test]
    fn replaces() {
        let mut set = HashSet::new();
        let word = "abc";

        super::replaces(word, &mut set);

        assert_eq!(set.len(), ((word.len()) * 26) - (word.len() - 1));
    }

    #[test]
    fn transposes() {
        let mut set = HashSet::new();
        let word = "abc";

        super::transposes(word, &mut set);

        assert_eq!(set.len(), word.len() - 1);
    }

    #[test]
    fn edit1() {
        let word = "somthing";

        let set = super::edits1(word);

        assert_eq!(set.len(), 442);
    }
}
