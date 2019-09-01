fn main() {
    borrow_matched();
    old_borrow_matched1();
    old_borrow_matched2();
}

fn borrow_matched() {
    let maybe_number = Some(42);
    // let maybe_number: Option<i32> = None;

    match &maybe_number {   // Reference match expression
        // Non-reference pattern
        // Match ergonomics will:
        // pattern-match the Option as a reference
        // bind the Option's value as a reference
        Some(borrowed) => {
            let x = *borrowed;
            println!("Found something: {}", x);
        },
        None => println!("Found nothing"),
    }
}

fn old_borrow_matched1() {
    let maybe_number = Some(42);
    // let maybe_number: Option<i32> = None;

    let maybe_number_ref = &maybe_number;

    match maybe_number_ref {    // Reference match expression
        // Reference pattern, explicit borrow using 'ref'
        &Some(ref borrowed) => {
            let x = *borrowed;
            println!("Found something: {}", x);
        },
        &None => println!("Found nothing"),
    }
}

fn old_borrow_matched2() {
    let maybe_number = Some(42);
    // let maybe_number: Option<i32> = None;

    let maybe_number_ref = &maybe_number;

    match *maybe_number_ref {   // Non-reference match expression
        // Non-reference pattern, explicit borrow using 'ref'
        Some(ref borrowed) => {
            let x = *borrowed;
            println!("Found something: {}", x);
        },
        None => println!("Found nothing"),
    }
}
