// a simple fibonacci sequence generator
// up until the nth number
use std::cmp::Ordering;
use std::io;
fn main() {
    loop {
        println!("Insert the Fibonacci number index:");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
        if index.trim().cmp("quit") == Ordering::Equal {
            println!("Quitting...");
            break;
        }
        let index: u32 = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please insert a number");
                continue;
            }
        };
        println!("Fibonacci sequence up until index {index}:");
        // compute the fibonacci sequence up until the nth number
        let mut fib: [u64; 3] = [0, 1, 1];
        let mut overflow: bool = false;
        for _i in 0..index {
            if overflow {
                println!("\nOverflow detected!");
                break;
            }
            print!("{} ", fib[0]);
            fib[0] = fib[1];
            fib[1] = fib[2];
            (fib[2], overflow) = fib[0].overflowing_add(fib[1]);
        }
        println!();
        println!("End of sequence!");
    }
}
