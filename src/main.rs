use rand::prelude::*;
use std::collections::HashMap;
use std::io::{prelude::*, Stdin};
#[derive(Debug)]

struct HangMan {
    word: Vec<char>,                     // state of the current word
    required: HashMap<char, Vec<usize>>, // hashmap which contains the map of char and positions in the word
    given: String,                       // the given string
    input: Stdin,                        // Standard input stream
}
impl HangMan {
    pub fn new(given: String) -> HangMan {
        let mut required = HashMap::<char, Vec<usize>>::new();
        let mut word: Vec<char> = given.chars().collect();
        let mut i = 0;
        let input = std::io::stdin();

        while i < word.len() {
            required
                .entry(word[i])
                .and_modify(|pos| pos.push(i))
                .or_insert_with(|| vec![i]);

            word[i] = '_';
            i += 2;
        }
        HangMan {
            word,
            required,
            given,
            input,
        }
    }
    fn readchar(&mut self) -> char {
        let mut buff = [0u8];
        self.input
            .read_exact(&mut buff)
            .expect("Failed to read input");
        char::from(buff[0])
    }
    pub fn play(&mut self) -> bool {
        let mut c = self.readchar();
        if c == '\n' {
            c = self.readchar();
        }
        if self.required.contains_key(&c) {
            let pos = self.required.get(&c).unwrap();
            for i in pos {
                self.word[*i] = c;
            }
            self.required.remove(&c);
            true
        } else {
            false
        }
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
                 / \  |
                ______|_"#,
        ];
        let can = 7;
        let mut i = 0;
        while i < can && !self.required.is_empty() {
            println!(
                "{}\n Word :{}",
                man[i],
                self.word.iter().collect::<String>()
            );
            println!("Entr YOur choice :");
            if !self.play() {
                i += 1;
            }
        }
        if i < can {
            println!("The word is {}", self.given);
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
