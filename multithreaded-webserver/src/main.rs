use std::{
  io::{prelude::*, BufReader},
  net::{TcpListener, TcpStream},
  process,
  env,
  fs,
  thread,
  time::Duration,
};
use parser::Config;

const HELP: &'static str = "
webserver establishes a multithreaded webserver.
Usage: 
  webserver <SERVER_ADDRESS>

Options:
  -h, --help      print this help menu
  --version       print version
";

fn main() {
  let config = match Config::new(env::args(), HELP, "webserver") {
    Ok(config) => config,
    Err(e) => {
      eprintln!("{}", e);
      process::exit(1); /* exit with error code 1 */
    },
  };
  let listener = TcpListener::bind(&config.server_address).unwrap();
  for incoming_stream in listener.incoming() {
    let stream = incoming_stream.unwrap();
    println!("Connection established!");
    handle_connection(stream, &config);
  } 
}

fn handle_connection(mut stream: TcpStream, config: &Config) {
  let buf_reader = BufReader::new(&mut stream);
  let request_line = buf_reader.lines().next().unwrap().unwrap();
  match request_line.as_str() {
    "GET / HTTP/1.1" => {
      //thread::sleep(Duration::from_secs(5));
      post_request(
        &mut stream, 
        "HTTP/1.1 200 OK",
        &config.html_page
      );
    }
    _ => post_request(
      &mut stream, 
      "HTTP/1.1 404 NOT FOUND",
      &config.error_page
    )
  }
}

fn post_request(stream: &mut TcpStream, status_line: &str, filepath: &str) {
  let contents = fs::read_to_string(filepath)
    .unwrap();
  let length = contents.len();  
  let response = format!("\
    {status_line}\r\nContent-Length: \
    {length}\r\n\r\n{contents}"
  );
  stream.write_all(response.as_bytes())
    .unwrap();
}


mod parser {
  use std::env;  
  pub struct Config {
    pub server_address: String,
    pub html_page: String,
    pub error_page: String,
    pub program_name: String,
  }
  impl Config {
    pub fn new(
      mut args: impl Iterator<Item = String>, 
      help: &'static str,
      program_name: &str,
    ) -> Result<Config, &'static str> {
        args.next();
        let server_address = match args.next() {
          Some(arg) => {
            match arg.as_str() {
              "-h" | "--help" => return Err(help),
              "--version" => {
                let vers = format!("{} v{}", program_name, env!("CARGO_PKG_VERSION"));
                return Err(string_to_static_str(vers));
              },
              _ => arg,
            }
          },
          None => return Err(help),
        };
        let html_page = match args.next() {
          Some(arg) => {
            match arg.as_str() {
              "-h" | "--help" => return Err(help),
              "--version" => {
                let vers = format!("{} v{}", program_name, env!("CARGO_PKG_VERSION"));
                return Err(string_to_static_str(vers));
              },
              _ => arg,
            }
          },
          None => return Err(help),
        };
        let error_page = match env::var("PAGE_404") {
          Ok(val) => val,
          Err(_) => String::from("./page/404.html")
        };
        Ok(Config {
          server_address,
          html_page,
          error_page,
          program_name: String::from(program_name)
        })
      }
  }
  /* WARNING: Unsafe! Leaks String memory to mmake it static */
  fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
  }
}