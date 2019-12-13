use std::collections::HashMap;

pub trait Markov {
    fn add_data(&mut self, data: &str) {
        let words = data.split(" ");
        let words_skip = words.clone().skip(1);
        for (first, second) in words.zip(words_skip) {
            self.add_pair(first, second);
        }
    }
    fn add_pair(&mut self, first: &str, second: &str);
    fn probability(&self, first: &str, second: &str) -> f64;
    fn run(&self, n: u32) -> &str;
    fn words(&self) -> Vec<String>;
}

struct MarkovHash { data: HashMap<String,  HashMap<String, u32>> }
impl Markov for MarkovHash {
    fn add_pair(&mut self, first: &str, second: &str) {
        if let Some(words) = self.data.get_mut(first) {
            words.insert(second.to_owned(), match words.get(second) {
                Some(c) => c + 1,
                None => 1,
            });
        } else {
            let mut words = HashMap::new();
            words.insert(second.to_owned(), 1);
            self.data.insert(first.to_owned(), words);
        }
    }

    fn probability(&self, first: &str, second: &str) -> f64 {
        0.0
    }

    fn run(&self, n: u32) -> &str {
        "gobbledygook"
    }

    fn words(&self) -> Vec<String> {
        self.data.keys().map(|s| {s.clone()}).collect()
    }
}

fn main() {
    println!("Hello, world!");
}
