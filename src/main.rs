use rand::prelude::*;
use std::collections::HashMap;
use std::io::prelude::*;
#[derive(Debug)]
struct HangMan {
    word: Vec<char>,
    required: HashMap<char, usize>,
    given: String,
}
impl HangMan {
    // add code here
    pub fn new(given: String) -> HangMan {
        let mut donec: Vec<char> = Vec::new();
        let mut required: HashMap<char, usize> = HashMap::new();
        let mut word: Vec<char> = given.chars().collect();
        let mut i: usize = 0;
        while i < word.len() {
            if donec.contains(&word[i]) {
                i += 1;
            } else {
                required.insert(word[i], i);
                word[i] = '0';
                donec.push(word[i]);
                i += 2;
            }
        }
        HangMan {
            word,
            required,
            given,
        }
    }
    fn readchar(&self) -> char {
        let mut input = std::io::stdin();
        let mut buff = [0u8];
        input.read_exact(&mut buff).expect("Failed to read input");
        let a = String::from_utf8_lossy(&buff);
        let a: Vec<char> = a.chars().collect();
        a[0]
    }
    pub fn play(&mut self) -> bool {
        // println!("Begin Play");
        let mut c = self.readchar();
        if c == '\n' {
            c = self.readchar();
        }
        if self.required.contains_key(&c) {
            self.word[self.required[&c]] = c;
            self.required.remove(&c);
            true
        } else {
            false
        }
    }
    fn get_word(&self) -> String {
        let mut result = String::new();
        for i in 0..self.word.len() {
            if self.word[i] == '0' {
                result.push('_');
            } else {
                result.push(self.word[i]);
            }
        }
        result
    }
    pub fn run(&mut self) {
        let man = [
            r#"
    	           ____
                  |   |
                      |
                      |
                      |
                ______|"#,
            r#"
    	           ____ 
                  |   |
                  o   |
                      |
                      |
                ______|_"#,
            r#"
    	           ____ 
                  |   |
                 _o   |
                      |
                      |
                ______|_"#,
            r#"
    	           ____ 
                  |   |
                 _o_  |
                      |
                      |
                ______|_"#,
            r#"
    	           ____ 
                  |   |
                 _o_  |
                  |   |
                      |
                ______|_"#,
            r#"
    	           ____ 
                  |   |
                 _o_  |
                  |   |
                 /    |
                ______|_"#,
            r#"
                   ____ 
                  |   |
                 _o_  |
                  |   |
                 / \\ |
                ______|_"#,
        ];
        let can = 7;
        let mut i = 0;
        while i < can && !self.required.is_empty() {
            println!("{}\n Word :{}", man[i], self.get_word());
            println!("Entr YOur choice :");
            if !self.play() {
                i += 1;
            }
        }
        if i < can {
            println!("Congratulations you won");
        } else {
            println!("Try again the word was {}", self.given);
        }
    }
}

fn main() {
    let data: Vec<&str> = include_str!("data.txt").split('\n').collect();
    let mut rng = rand::thread_rng();
    let i: usize = rng.gen_range(0..data.len());
    let mut game = HangMan::new(String::from(data[i]));
    game.run();
}
