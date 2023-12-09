use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
  pub file_path: String,
  pub pattern: String,
  pub case_insensitive: bool,
}

impl Config {
  pub fn new(mut args: impl Iterator<Item = String>, help: &'static str) -> Result<Config, &'static str> {
    args.next();
    let file_path = match args.next() {
      Some(arg) => {
        match arg.as_str() {
          "-h" | "--help" => return Err(help),
          "--version" => {
            let vers = format!("mini-grep v{}", env!("CARGO_PKG_VERSION"));
            return Err(string_to_static_str(vers));
          },
          _ => arg,
        }
      },
      None => return Err(help),
    };
    let pattern = match args.next() {
      Some(arg) => arg,
      None => return Err(help),
    };
    let case_insensitive: bool = match env::var("CASE_INSENSITIVE") {
      Ok(val) => val == "1",
      Err(_) => false,
    };
    Ok(Config {
      file_path: file_path, /* inneficent solution: use clone() */ 
      pattern: pattern, /* efficient solution: use reference with lifetime annotation */
      case_insensitive
    })
  }
}

/* WARNING: Unsafe! Leaks String memory to mmake it static */
fn string_to_static_str(s: String) -> &'static str {
  Box::leak(s.into_boxed_str())
}

fn search_case_sensitive(pattern: &str, contents: &str) -> HashMap<(usize, usize), String> {
  let mut lines: HashMap<(usize, usize), String> = HashMap::new();
  contents.lines()
    .filter(|line| line.contains(pattern))
    .for_each(|line| {
      let cursor_pos = contents.find(line).unwrap();
      let line_index = contents[..cursor_pos].lines().count();
      let column_index = line.find(pattern).unwrap();
      lines.insert( (line_index, column_index), line.to_string());
    });
  lines
}

fn search_case_insensitive(pattern: &str, contents: &str) -> HashMap<(usize, usize), String> {
  let mut lines: HashMap<(usize, usize), String> = HashMap::new();
  contents.lines()
    .filter(|line| {
      line.to_lowercase()
        .contains(pattern.to_lowercase().as_str())
    })
    .for_each(|line| {
      let cursor_pos = contents.find(line).unwrap();
      let line_index = contents[..cursor_pos].lines().count();
      let column_index = line.to_lowercase()
        .find(pattern.to_lowercase().as_str()).unwrap();
      lines.insert( (line_index, column_index), line.to_string());
    });
  lines
}

pub fn run_mini_grep(config: Config) 
-> Result<HashMap<(usize, usize), String>, Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;
  /* ? will return the error value from the current 
    function for the caller to handle. 
  */
  let lines = if config.case_insensitive {
    search_case_insensitive(&config.pattern, &contents)
  } else {
    search_case_sensitive(&config.pattern, &contents)
  };
  Ok(lines)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn search_case_sensitive_result_content() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    let found = search_case_sensitive(query, contents);
    let found_contents = found.values().cloned().collect::<Vec<String>>();
    assert_eq!(
      vec!["safe, fast, productive."], 
      found_contents
    );
  }

  #[test]
  fn search_case_sensitive_result_line_column() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    let found = search_case_sensitive(query, contents);
    let found_line_cols = found.keys().cloned().collect::<Vec<(usize,usize)>>();
    assert_eq!(
      vec![(1, 15)], 
      found_line_cols
    );
  }

  #[test]
  fn search_case_insensitive_result_content() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Trust me.
Pick three.";
    let found = search_case_insensitive(query, contents);
    let mut found_contents = found.values().cloned().collect::<Vec<String>>();
    /* sort results by string length to get a deterministic answer */
    found_contents.sort_by(|a, b| a.len().cmp(&b.len()));
    assert_eq!(
      vec!["Rust:", "Trust me."],
      found_contents
    );
  }

  #[test]
  fn search_case_insensitive_result_line_column() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Trust me.
Pick three.";
    let found = search_case_insensitive(query, contents);
    let mut found_line_cols = found.keys().cloned().collect::<Vec<(usize, usize)>>();
    /* sort by line, column pair to get a determinisic behaviour*/
    found_line_cols.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    assert_eq!(
      vec![(0,0), (2,1)], 
      found_line_cols
    );
  }
}