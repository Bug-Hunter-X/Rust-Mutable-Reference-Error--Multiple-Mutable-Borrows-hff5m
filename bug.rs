fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x

    *y = 6; // Modify x through y
    *z = 7; // Modify x through z

    println!("x = {}", x); // x will be 7
}