//! Storing data on the Heap with Box<T>
//! When to use Box<T>:
//! - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
//! - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
//! - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
use std::rc::Rc;
use std::cell::RefCell;
/* Lisp language Cons List */
#[derive(Debug)]
enum Cons {
  Cons(i32, Box<Cons>),
  Nil,
}
#[derive(Debug)]
enum NewCons {
    Cons(i32, Rc<NewCons>),
    Nil,
}
#[derive(Debug)]
enum InternallyMutableCons {
    Cons(Rc<RefCell<i32>>, Rc<InternallyMutableCons>), /* RefCell<T> is a type of smart pointer that enforces the borrowing rules at runtime instead of compile time */
    Nil,                             /* RefCell<T> enables an immutable object within the scope to be mutated outside of that scope */
}

fn main() {
  let list_a = Cons::Cons(1, Box::new(Cons::Cons(2, Box::new(Cons::Cons(3, Box::new(Cons::Nil))))));
  let list_b = Cons::Cons(4, Box::new(list_a));
  println!("list_a: {:?}", list_b);
  println!("list_b: {:?}", list_b);
  /* Forbidden */
  // let list_c = Cons::Cons(5, Box::new(list_a)); /* more than one references pointing to list_a's address */
  /* must use Rc<T> _: Reference Counted references */
  let list_a = Rc::new(NewCons::Cons(1, Rc::new(NewCons::Cons(2, Rc::new(NewCons::Cons(3, Rc::new(NewCons::Nil)))))));
  let list_b = NewCons::Cons(4, Rc::clone(&list_a));
  let list_c = NewCons::Cons(5, Rc::clone(&list_a));
  println!("list_a: {:?}", list_a);
  println!("list_b: {:?}", list_b);
  println!("list_c: {:?}", list_c);
  /* Internal Mutability */
  let value = Rc::new(RefCell::new(5));
  let list_a = Rc::new(InternallyMutableCons::Cons(Rc::clone(&value), Rc::new(InternallyMutableCons::Nil)));
  let list_b = Rc::new(InternallyMutableCons::Cons(Rc::new(RefCell::new(6)), Rc::clone(&list_a)));
  let list_c = InternallyMutableCons::Cons(Rc::new(RefCell::new(7)), Rc::clone(&list_b));
  println!("list_a: {:?}", list_a);
  println!("list_b: {:?}", list_b);
  println!("list_c: {:?}", list_c);
  *value.borrow_mut() += 10; /* this will update the first value of each Cons List */
  println!("list_a: {:?}", list_a);
  println!("list_b: {:?}", list_b);
  println!("list_c: {:?}", list_c);
}
