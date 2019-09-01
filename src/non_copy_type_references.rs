fn main() {
    borrow_matched();
    old_borrow_matched1();
    old_borrow_matched2();
}

fn borrow_matched() {
    // Uses match ergonomics
    let maybe_number = Some(Box::new(42));
    // let maybe_number: Option<Box<i32>> = None;

    match &maybe_number {   // Reference match expression
        // Non-reference pattern
        // Match ergonomics will:
        // pattern-match the Option as a reference
        // bind the Option's value as a reference
        Some(borrows_box) => {
            // Dereference borrow, then dereference Box
            let x = **borrows_box;
            println!("Found something: {}", x);
        },
        None => println!("Found nothing"),
    }

    // Can match again - value was borrowed
    match &maybe_number {
        Some(_) => {},
        None => {},
    }
}

#[allow(unused)]
fn borrow_matched_error() {
    let maybe_number = Some(Box::new(42));
    // let maybe_number: Option<Box<i32>> = None;

    match &maybe_number {
        Some(borrows_box) => {
            // Compiler error - move out of box into a new variable
            // This can't be done because the match only borrows the value.
            /*
            let the_box = *borrows_box;
            let x = *the_box;
            println!("Found something: {}", x);
            */
        },
        None => println!("Found nothing"),
    }
}

#[allow(unused)]
fn old_borrow_matched_error() {
    let maybe_number = Some(Box::new(42));
    // let maybe_number: Option<Box<i32>> = None;

    let maybe_number_ref = &maybe_number;

    // Compiler error - cannot move out of borrowed content
    /*
    match maybe_number_ref {
        // Binds the Option as a reference but tries to own the box.
        // Can't move from behind a reference
        &Some(owns_box) => {},
        &None => {},
    }
    */
}

fn old_borrow_matched1() {
    let maybe_number = Some(Box::new(42));
    // let maybe_number: Option<Box<i32>> = None;

    let maybe_number_ref = &maybe_number;

    match maybe_number_ref {    // Reference match expression
        // Reference pattern, explicit borrow using 'ref'
        &Some(ref borrows_box) => {
            // Dereference borrow, then dereference Box
            let x = **borrows_box;
            println!("Found something: {}", x);
        },
        &None => println!("Found nothing"),
    }
}

fn old_borrow_matched2() {
    let maybe_number = Some(Box::new(42));
    // let maybe_number: Option<Box<i32>> = None;

    let maybe_number_ref = &maybe_number;

    match *maybe_number_ref {   // Non-reference match expression
        // Non-reference pattern, explicit borrow using 'ref'
        Some(ref borrows_box) => {
            // Dereference borrow, then dereference Box
            let x = **borrows_box;
            println!("Found something: {}", x);
        },
        None => println!("Found nothing"),
    }
}
