use std::{
  io::{prelude::*, BufReader},
  net::{TcpListener, TcpStream},
  process,
  env,
  fs,
  thread,
  time::Duration,
  sync::Arc
};
use parser::Config;
use myhttpserver::ThreadPool;

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
  let mut pool = ThreadPool::new(config.pool_size);
  let config = Arc::new(config); /* share a reference to config between multiple threads */
  for incoming_stream in listener.incoming().take(2) {
    let stream = incoming_stream.unwrap();
    let config = Arc::clone(&config);
    println!("Connection established!");
    pool.execute(move || {
      handle_connection(stream, &config)
    });
  } 
}

fn handle_connection(mut stream: TcpStream, config: &Config) {
  let buf_reader = BufReader::new(&mut stream);
  let request_line = buf_reader.lines().next().unwrap().unwrap();
  match request_line.as_str() {
    "GET / HTTP/1.1" => post_request(
      &mut stream, 
      "HTTP/1.1 200 OK",
      &config.html_page
    ),
    "GET /sleep HTTP/1.1" => thread::sleep(Duration::from_secs(5)),
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

mod myhttpserver {
  use std::{
    thread,
    sync::{mpsc, Arc, Mutex}
  };

  pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
  }
  impl Drop for ThreadPool {
    /* all threads should join a locked state 
    to finish their jobs before being droped 
    by the main thread. */
    fn drop(&mut self) {
      drop(self.sender.take()); // drop sender of the channel
      for worker in &mut self.workers {
        println!("Shutting down worker {}", worker.id);
        /* call take on option to get the val in Some(val) and 
        leave None value in the place of Option<thread::JoinHandle<()>. */
        if let Some(thread) = worker.thread.take() {
          thread.join().unwrap();
        }
      }
    }
  }
  impl ThreadPool {
    pub fn new(pool_size: usize) -> Self {
      assert!(pool_size>0);
      let mut workers = Vec::with_capacity(pool_size);
      let (sender, receiver) = mpsc::channel();
      let receiver = Arc::new(Mutex::new(receiver));
      for id in 0..pool_size {
        workers.push(Worker::new(id, Arc::clone(&receiver)))
      }
      let sender = Some(sender);
      ThreadPool { workers, sender }
    }
    pub fn execute<F>(&mut self, fun: F) 
    where
      F: FnOnce() + Send + 'static
    {
      let job = Box::new(fun);
      self.sender.as_ref().unwrap().send(job).unwrap();
    }
  }
  pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
  }
  impl Worker {
    /* use Arc because we need a reference pointer that can be shared between multilpe threads to the same channel receiver */
    pub fn new<'a>(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
      let thread = thread::spawn(move || {
        loop { /* loop waiting for new jobs */
          let received_message = receiver.lock().unwrap().recv();
          match received_message {
            Ok(job) => {
              println!("Worker {id} got a job; executing.");
              job();
            },
            Err(_) => {
              println!("Worker {id} disconnected; shutting down.");
              break; /* exit loop if receiver/sender were droped causing the channel to close. */
            } 
          }
        }
      });
      let thread = Some(thread);
      Worker { id, thread }
    }
  }
  type Job = Box<dyn FnOnce() + Send + 'static>;
}
mod parser {
  use std::env;  
  pub struct Config {
    pub server_address: String,
    pub html_page: String,
    pub error_page: String,
    pub pool_size: usize,
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
        let html_page = args.next().unwrap();
        let pool_size= match args.next() {
            Some(arg) => arg.parse().unwrap(),
            _ => 5
        };
        let error_page = match env::var("PAGE_404") {
          Ok(val) => val,
          Err(_) => String::from("./page/404.html")
        };
        Ok(Config {
          server_address,
          html_page,
          error_page,
          pool_size,
          program_name: String::from(program_name)
        })
      }
  }
  /* WARNING: Unsafe! Leaks String memory to mmake it static */
  fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
  }
}