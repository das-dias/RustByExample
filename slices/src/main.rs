// this program gets the input
// from the user, and separates
// the input in each space found
use std::io;

fn main() {
    loop {
        println!("Enter your string: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // find the first each word in the
        // input string
        if input.trim() == "quit" {
            println!("Exiting...");
            break;
        }
        println!("The found inputs are:");
        let mut i = 0;
        while i < input.len() {
            let (word, j) = first_word(&input[i..]);
            if !(word.trim().is_empty()) {
                println!("{word}");
            }
            i += j + 1;
        }
    }
}

// find the first word before a space in a string
fn first_word(s: &str) -> (&str, usize) {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[0..i], i);
        }
    }
    (s, s.len())
}
