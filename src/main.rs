fn main() {
    let x = String::from("Hello!");
    let y = &x;

    println!("x = {}", x);
    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("y = {}", *y);
    drop(x);
}
