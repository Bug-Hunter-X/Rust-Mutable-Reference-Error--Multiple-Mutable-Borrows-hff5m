use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    let y = x.borrow_mut(); // y is a mutable reference (only one allowed)
    *y = 6;
    println!("x = {}", x.borrow()); // x is now 6
    let z = x.borrow_mut();
    *z = 7;
    println!("x = {}", x.borrow()); // x is now 7
} 