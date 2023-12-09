use std::env;
use std::fs;

const HELP: &'static str = "
grep finds a string pattern in a file.
Usage: 
  grep <FILEPATH> <PATTERN>

Options:
  -h, --help      print this help menu
  --version       print version
";

fn main() {
  let args: Vec<String> = env::args().collect();
  // dbg!(&args);
  let mut filePath: &String;
  let mut pattern: &String;
  match args.len() {
    1 => {println!("{}", &HELP); return;},
    2 => {
      let arg = &args[1];
      match arg.as_str() {
        "-h" | "--help" => println!("{}", &HELP),
        "--version" => println!("0.0.1"),
        _ => println!("{}", &HELP),
      }
      return;
    },
    3 => {
      filePath = &args[1];
      pattern = &args[2];
      println!("filepath: {}", filePath);
      println!("pattern: {}", pattern);
    },
    _ => {println!("{}", &HELP); return;},
  }
  let contents = fs::read_to_string(filePath)
    .expect("Something went wrong reading the file");
  println!("With text:\n{}", contents);
}
