fn main() {
    let maybe_number = Some(42);
    // let maybe_number: Option<i32> = None;

    match maybe_number {
        Some(x) => println!("Found something: {}", x),
        None => println!("Found nothing"),
    }
}
