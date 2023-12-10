/* References are only valied within the 
 scope of their lifetime. If they leave 
 that scope, their lifetime ends. 
*/

fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
  /* x and y are in different scopes, and the function 
    must know where it is borrowing the variable from (either 
    form x or from y ). */
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
fn main() {
  let string1 = String::from("abcd");
  let string2 = "xyzhihi";
  let result = longest_string(string1.as_str(), string2);
  println!("The longest string is {}", result);
}
