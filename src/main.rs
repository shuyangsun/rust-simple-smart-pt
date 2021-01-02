use std::rc::Rc;

fn main() {
    let val = Rc::new(String::from("hello"));
    println!("val = {}", val);
}
