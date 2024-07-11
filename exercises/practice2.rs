fn main() {
    // The `mut` keyword lets the variable be changed
    let mut a_number = 10;
    println!("The number is {}.", a_number);

    // Change the value of an immutable variable
    a_number = 15;
    println!("Now the number is {}.", a_number);
}
