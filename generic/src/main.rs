/* generic data types */

// non-generic implementation
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic implementation
fn largest<T>(list: &[T]) -> &T 
where T: std::cmp::PartialOrd { /* trait bound to ensure that the generic type can be compared */
  let mut largest: &T = &list[0];
  list.iter().for_each(|item: &T| {
    if item > largest {largest = item;}
  });
  largest
}
#[allow(non_snake_case)]
fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest_i32(&number_list);
  let secondResult = largest(&number_list);
  println!("The largest number is {}", result);
  println!("The largest numbers are equal: {}", secondResult == result);  
  let char_list = vec!['y', 'm', 'a', 'q'];
  let result = largest_char(&char_list);
  let secondResult = largest(&char_list);
  println!("The largest char is {}", result);
  println!("The largest chars are equal: {}", secondResult == result);
}