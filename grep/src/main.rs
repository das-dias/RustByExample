use std::env;
use std::process;

use grep::Config; /* import local module */

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
  let config = match Config::new(&args, &HELP.to_string()) {
    Ok(config) => config,
    Err(e) => {
      eprintln!("{}", e);
      process::exit(1); /* exit with error code 1 */
    },
  };
  let contents_result = grep::run_mini_grep(config).unwrap_or_else( |err| {
    eprintln!("mini-grep error: {}", err);
    process::exit(1);
  });
  match contents_result.len() {
    0 => eprintln!("No matches found."),
    _ => {
      println!("Line/Column\tContent");
      for (line_column_tuple, line) in contents_result {
        let line_number = line_column_tuple.0;
        let column_number = line_column_tuple.1;
        println!("l{}/c{}\t{}", line_number, column_number, line);
      }
    },
  }
}
