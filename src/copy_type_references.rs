fn main() {
    let maybe_number = Some(42);
    // let maybe_number: Option<i32> = None;

    let maybe_number_ref = &maybe_number;

    match maybe_number_ref {    // Reference match expression
        Some(x) => {            // Non-reference pattern; bound as ref by default
            println!("Found something: {}", x);
        },
        None => println!("Found nothing"),
    }
}
