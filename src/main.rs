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
}
