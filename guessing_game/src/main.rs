use std::{io, env, cmp::Ordering};
use rand::Rng;

#[allow(non_snake_case)]
fn main() {
  let args: Vec<String> = env::args().collect();
  dbg!(&args); // dbg! is a macro that prints the value of a variable
  let mut lowerRange: i32 = 1;
  let mut upperRange: i32 = 101;
  match args.len() {
    1 => println!("No arguments passed. Using default range of 1-100"),
    2 => {
        lowerRange = String::from(&args[1]).parse().expect("Lower range is an i32!");
        upperRange = 101;
        if lowerRange > upperRange {
          println!("Lower range cannot be greater than upper range.");
          lowerRange = 1;
          upperRange = 101;
        }
        println!("Hello, world! Using range {}-{}", lowerRange, upperRange);
    },
    3 => {
      lowerRange = String::from(&args[1])
                          .parse()
                          .expect("Lower range is an i32!");
      upperRange = String::from(&args[2])
                          .parse()
                          .expect("Upper range is an i32!");
      if lowerRange > upperRange {
        println!("Lower range cannot be greater than upper range.");
        lowerRange = 1;
        upperRange = 101;
      }
      println!("Hello, world! Using range {}-{}", lowerRange, upperRange);
    },
    _ => println!("Too many arguments passed. Using default range of 1-100"),
  }
  println!("Guess the number: ");
  let mut guess: String = String::new();
  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
  println!("You guessed: {}", guess);
  let secretNumber: i32 = rand::thread_rng()
                                .gen_range(lowerRange..upperRange);
  let guessedNumber: i32 = guess.trim()
                                .parse()
                                .expect("Please type a number!");
  println!("The secret number is: {}", secretNumber);
  match guessedNumber.cmp(&secretNumber) {
  Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
  }
}