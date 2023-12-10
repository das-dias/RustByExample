// a simple program to convert temperature between
// Celsius and Kelvin
use std::cmp::Ordering;
use std::io;
const ABSOLUTE_ZERO_C: f64 = -273.15;
fn main() {
    loop {
        println!("Enter a temperature in Celsius:");
        let mut celsius = String::new();
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");
        if celsius.trim().cmp("quit") == Ordering::Equal {
            println!("Quitting...");
            break;
        }
        let celsius: f64 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        if celsius < ABSOLUTE_ZERO_C {
            println!("The temperature cannot be below absolute zero!");
            continue;
        }
        let kelvin = celsius + 273.15;
        println!("{}°C is {}°K", celsius, kelvin);
    }
}
