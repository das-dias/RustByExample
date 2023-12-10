//! Fearless concurrency with Rust
//! Tackling problems such as:
//! - Race conditions, where threads are accessing data or resources in an inconsistent order
//! - Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
//! - Bugs that happen only in certain situations and are hard to reproduce and fix reliably
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
  /* # When the main thread finishes, the spawns all go back to hell
  never finishing their count completely. */
  thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });
  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  /* # Using handles to wait for the threads to finish */
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the handled thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });
  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
  // block the thread from exiting until it finishes
  handle.join().unwrap();

  /* # Calling join on a thread bfore the main thread will 
  block the main thread from executing */
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the blocking handled thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });
  handle.join().unwrap();
  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  /* # Moving values into threads: use move to tell
  the main thread its variable was moved into another scope, 
  to prevent its destruction before the thread ends. */
  let v = vec![1, 2, 3];
  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });
  handle.join().unwrap();

  /* Message Paassing to Transmit Data between Threads */
  /* multiple producer, single consumer (mpsc) */
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
  });
  let received = rx.recv().unwrap();
  println!("Got: {}", received);
  
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });
  /* not using recv, and treating receiver as an iterator
  blocks the main thread while waiting for new messeges */
  for received in rx {
    println!("Got: {}", received);
  }

  /* cloning the transmitter to have more producers */
  let (tx, rx) = mpsc::channel();
  let tx1 = tx.clone();
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];
    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });
  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });
  for received in rx {
    println!("Got: {}", received);
  }
  /* # Mutexes (or Mutual Exclusion Data Access) */
  /* Mutexes have a reputation for being difficult to use because you have to remember two rules:
  You must attempt to acquire the lock before using the data.
  When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock. 
  */
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];
  for _ in 0..10 {
    /* use an Atomic Reference Counted Pointer clone to mutate
    the main thread's counter inside the thread-spawned closure. */
    let counter2 = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter2.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
  }
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Result: {}", *counter.lock().unwrap());
}
