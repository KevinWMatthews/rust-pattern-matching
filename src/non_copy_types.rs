fn main() {
    own_matched();
    old_borrow_matched();
}


fn own_matched() {
    let maybe_number = Some(Box::new(42));
    // let maybe_number: Option<Box<i32>> = None;

    match maybe_number {
        Some(owns_box) => {
            let x = *owns_box;  // Dereference Box
            println!("Found something: {}", x);
        },
        None => println!("Found nothing"),
    }

    // Compiler error - use after move
    /*
    match maybe_number {
        Some(_) => {},
        None => {},
    }
    */
}

fn old_borrow_matched() {
    // Deprecated by match ergonomics? Might be better to use:
    // match &maybe_number

    let maybe_number = Some(Box::new(42));
    // let maybe_number: Option<Box<i32>> = None;

    match maybe_number {
        Some(ref borrows_box) => {  // Explicitly bind Box as a reference
            let x = **borrows_box;
            println!("Found something: {}", x);
        },
        None => println!("Found nothing"),
    }
}
