use std::{array, f32::consts, panic, sync::atomic::{AtomicBool, Ordering}};

static FAIL_SAFE_MODE: AtomicBool = AtomicBool::new(false);
static mut COUNTER: u32 = 0;
const MAX_VALUE: u32 = 100;

fn main() {
    let x: Option<&i32> = None;
    println!("{:?}", x);

    // allocates heap memory to store integer 42
    let original_ptr = Box::new(42);

    // borrow smart pointer
    let copied_ptr = &original_ptr;

    // attempt to drop original_ptr
    // drop(original_ptr); >> doesn't compile

    // borrowed smart pointer later used here
    println!("value: {}", copied_ptr);

    let (mut _a, mut _b, mut _c) = (10_u8, 20u32, 0);

    println!("{:?}, {:?}, {:?}", _a, _b, _c);

    let ascii_code_of_plus = b'+';
    println!("ASCII Code of '+' : {}", ascii_code_of_plus);

    let numeric_value = 43;
    println!("43 is {}", numeric_value as u8 as char);

    let x = b'+'; 
    // x type is u8
    // x consumes 1 byte
    // ASCII code of '+' is stored in x which is 43
    
    let y = '+';
    // y type is char
    // in rust char type consumes 4 bytes
    // Unicode Scalar Value (USV) of '+' is stored in y
    // USV of character '+' = U+002B (decimal 43)

    let infinity_symbol = '\u{221E}';
    println!("symbol = {}, usv = {}", infinity_symbol, infinity_symbol as u32);

    let tm = '\u{00AE}';
    println!("trade mark: {}", tm);

    let usv_of_inf = 0x221e_u32;

    if let Some(inf_symbol) = char::from_u32(usv_of_inf) {
        println!("symbol = {}", inf_symbol);
    } else {
        println!("not a valid unicode scalar value");
    }

    /*
    unsafe {
        COUNTER += 1;
        println!("counter: {}", COUNTER);
    }
    */


    // give custom panic hook
    panic::set_hook(Box::new(|info| {
        FAIL_SAFE_MODE.store(true, Ordering::SeqCst);
        println!("Panic occurred: {}", info);
        println!("Entering fail-safe mode...");
    }));

    // wrap the potentially panicking code in catch_unwind
    let result = panic::catch_unwind(|| {
        let buffer = [1, 2, 3, 4, 5];

        for i in 0..10 {
            // panics for i >= 5
            println!("Accessing index {}: {}", i, buffer[i]);
        }
    });

    if FAIL_SAFE_MODE.load(Ordering::SeqCst) {
        println!("System is now in fail-safe mode.");
    }

    match result {
        Ok(_) => println!("No panic occurred."),
        Err(_) => println!("Panic caught! Execution continues."),
    }

    let my_array = [2.5, 4.0, 3.8];
    let your_array = ['+', '-'];
    let his_array = [1, 2, 3_u8, 67, 89];

    println!("{:?}", your_array);

    let buffer = [0_u8; 10];
    let memory = [4.5; 7];
    println!("{:?}", buffer);
    println!("{:?}", memory);

    let her_array = [10, -67, 88, -5, -2, 99, 132, 32];

    let mut sum = 0;

    for num in her_array {
        sum += num;
    }

    println!("sum: {}", sum);

    // reference
    let value = 42;
    let ref_of_value = &value;
    println!("value is {}", *ref_of_value); // manual dereferencing
    println!("value is {}", ref_of_value); // automatic dereferencing
    println!("value is {:p}", ref_of_value); // referenced memory address
    println!("value is {:p}", &ref_of_value); // reference itself memory address

    // borrow
    let mut num1 = 50; // mutable referent
    let ref_of_num1 = &mut num1; // immutable borrow
    *ref_of_num1 = 100;
    println!("num1 : {}", num1);

    let num2 = 120; // immutable referent
    let ref_of_num2 = &num2;
    println!("num2 : {}", ref_of_num2);

    // there can be multiple immutable borrows
    // but not in the case of mutable borrows
    // and you cannot mix immutable borrow with mutable borrow

    // slice - a borrowed portion of contiguous data

    let mut the_array: [i32; 5] = [10, 22, -3, 94, 100];

    // let s1 = &the_array[1..3]; // items 2 and 3
    // let s2 = &the_array[..];
    // let s3 = &the_array[0..=1];
    let s4 = &the_array[2..4];
    let mut sum = 0;

    // println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);
    // s4[0] = 336;
    println!("the array: {:?}", the_array);
    println!("s4: {:?}", s4);

    for i in s4 {
        sum += i;
    }

    println!("sum : {}", sum);

    // if - else
    let age = 19;

    let message = if age < 18 {
        println!("this is if block");
        // "you cannot vote"
        100;
    } else {
        println!("this is else block");
        // "you still not voting because systems is rigged^^"
        2000; // returns unit type ()
    };

    println!("{:?}", message);

    // match
    let point = (3, -7);

    match point {
        (_, y) if y < 0 => {
            println!("Second element is negative: {}", y);
            println!("Take action 1");
        }
        (0, 0) => {
            println!("Point is zero");
            println!("Take action 2");
        }
        _ => println!("All fine"),
    }

    let array1 = [1, 2, 3, -1];

    let invalid_array = match array1 {
        [n,_,_,_] | [_,n,_,_] |
        [_,_,n,_] | [_,_,_,n] if n < 0 => {
            true
        }
        _ => false,
    };

    if invalid_array {
        println!("Array is invalid");
    } else {
        println!("Array is valid");
    }

    // match macro >> matches!(expression, pattern)
    let array2 = [5, 6, 7, -8];
    let array_validity = matches!(array2,
        [n,_,_,_] | [_,n,_,_] |
        [_,_,n,_] | [_,_,_,n] if n < 0
    );

    if array_validity {
        println!("Array2 is invalid");
    } else {
        println!("Array2 is valid");
    }
    
}
