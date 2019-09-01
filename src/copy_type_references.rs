fn main() {
    borrow_matched();
    old_borrow_matched1();
    old_borrow_matched2();
}

fn borrow_matched() {
    let maybe_number = Some(42);
    // let maybe_number: Option<i32> = None;

    match &maybe_number {   // Reference match expression
        Some(x) => {        // Non-reference pattern; bound as ref by default
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
        &Some(ref x) => {       // Reference pattern; must explicitly borrow using 'ref'
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
        Some(ref x) => {        // Non-reference pattern; must explicitly borrow using 'ref'
            println!("Found something: {}", x);
        },
        None => println!("Found nothing"),
    }
}
