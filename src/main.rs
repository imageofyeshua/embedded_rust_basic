use core::slice;
use std::{array, f32::consts, panic, sync::atomic::{AtomicBool, Ordering}, thread};

static FAIL_SAFE_MODE: AtomicBool = AtomicBool::new(false);
static mut COUNTER: u32 = 0;
const MAX_VALUE: u32 = 100;

#[derive(PartialEq)]
enum CarStatus {
    Up(u32, i32, i32),
    Down{speed: u32},
    Pause(Point),
    Broken
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

struct Circle {
    radius: f32,
    angle: f32,
}

#[derive(Debug, Default)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, Default)]
struct Person {
    name: String,
    age: u8,
    is_male: bool,
    height: f32,
    initial: char
}

#[derive(Debug)]
struct Process {
    name: String,
    pid: u32,
    group: String,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn new_with_default() -> Self {
        Rectangle::default()
    }

    fn area(&self) -> f32 {
        (self.width * self.height) as f32
    }
}

impl Point {
    // method that borrows self immutably
    fn distance_from_origin(self: &Point) -> f32 {
        // sqrt(x2 + y2)
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // method that borrows self mutably
    fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    // method that takes the ownership of self
    fn into_tuple(self) -> (f32, f32) {
        (self.x, self.y)
    }

    // associated function
    fn from_tuple(coords: &(f32, f32)) -> Point {
        Point { x: coords.0, y: coords.1 }
    }
}


fn update_person_age(person: &mut Person, new_age: u8) {
    person.age = new_age;
    // (*person).age = new_age; // OK
}

fn print_shape_point(point: &Point) {
    println!("Point: x = {}, y = {}", point.x, point.y);
}

fn print_shape_circle(circle: &Circle) {
    println!("Circle: radius = {:.2}, angle = {:.2}", circle.radius, circle.angle);
}

#[allow(unused_variables)]
fn main() {

    let mut p = Person {
        name: String::from("Alice"),
        age: 25,
        ..Default::default()
    };

    println!("Name: {}, Age: {}", p.name, p.age);

    update_person_age(&mut p, 30);

    println!("Name: {}, Age: {}", p.name, p.age);

    let mut p = Point { x: 3.0, y: 4.0 };
    println!("distance: {}", p.distance_from_origin());
    p.translate(0.1, 0.2);
    println!("modified points: {:?}", p);

    let points = p.into_tuple();
    println!("Points as tuple: {:?}", points);

    println!("{:?}", p); // Error

    let tuple = (10.0, 20.0);
    let q = Point::from_tuple(&tuple);
    println!("Point from tuple: ({}, {})", q.x, q.y);

    println!("{:?}", Rectangle::new(30, 50));
    println!("{:?}", Rectangle::new_with_default());

    let rect = Rectangle::new(24, 59);
    println!("area: {}", rect.area());

    let mut current_car_status = CarStatus::Pause(Point {x: 0.0, y: 0.0});

    current_car_status = CarStatus::Up(100, 67, 78);

    // if Car is in pause
    if current_car_status == CarStatus::Broken {
        println!("Yo, car is broken");
    }

    current_car_status = CarStatus::Pause(Point {x: 23.0, y: 67.9});

    match current_car_status {
        CarStatus::Up(a, ..) => {
            println!("Car is moving up with speed {}", a);
        }

        CarStatus::Pause(Point {x, ..}) => {
            println!("Car is not moving and x is {}", x);
        }

        _ => println!("Car is not moving up"),
    }

    /*
    let user = Person::default();
    let p1 = Person {
    name: String::from("Daniel"),
    is_male: true,
    ..Default::default()
    };

    println!("{:?}", user);
    println!("{:?}", p1);

    let process1 = Process {
    name: String::from("Ping"),
    pid: 0x1234,
    group: String::from("Networking"),
    };
    println!("Process 1: {:#X?}", process1);

    let process2 = Process {
    name: String::from("Route"),
    ..process1
    };
    println!("Process 2: {:#X?}", process2);

    let process3 = Process {
    pid: 0x3456,
    group: String::from("Security"),
    ..process2
    };
    println!("Process 3: {:#X?}", process3);
    */
    /*
    let p = Point { x: 10, y: 20};
    let c = Circle { radius: 5.5, angle: 90.0 };

    // mismatched type
    // print_shape_point(&c);

    print_shape_point(&p);
    print_shape_circle(&c);

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

    // if - let 
    // a shorthand for a match statement with just one pattern

    let point = (2, 4);

    if let (_, y @ 1..=4) = point {
    println!("y = {} is withing the range 1..=4", y);
    } else  if let (0, 0) = point {
    println!("Point is at the origin");
    } else {
    println!("Something else here...");
    }

    // threads

    // immutable data
    let data = 42;

    /*
    let handle1 = thread::spawn(|| {
    println!("Thread 1 reads data: {}", data);
    });

    // impossible >> owndership expands the lifetime of thread 1
    let handle2 = thread::spawn(|| {
    println!("Thread 2 reads data: {}", data);
    });
    */

    // handle1.join().unwrap();
    // handle2.join().unwrap();

    // bit operation
    let x = 0b0000_1111_u8;
    let y = !x;
    println!("y: {:b}", y);

    let f = 0xFF;
    let g = 0xF;
    let h = f & g;
    println!("h: {:b}", h);

    let i = 0x1A;
    let j = 0xF;
    let k = i | j;
    println!("k: {:b}", k);

    let l = 0b0011_0011_u8;
    let m = l << 2;
    println!("m: {:b}", m);

    let n = 0x80_u8;
    let o = n >> 2;
    println!("o: {:b}", o);

    let p = 0x80_i16;
    let q = p >> 2;
    println!("q: {}", q);

    // string

    let mut greeting = String::from("Good morning");

    greeting.replace_range(5.., "evening"); 

    greeting.push_str(", world!");

    println!("{}", greeting);

    let mut message = String::new();
    message.push_str("Daniel, Park");
    println!("{}", message);

    let my_string = "Hello, there".to_string();

    let mut num_as_string = 3.1416.to_string();
    num_as_string.insert_str(0, "PI = ");
    println!("{}", num_as_string);

    let mut hiroo = "Hello, there".to_string();
    let whatsup = &hiroo[0..5];
    println!("{}", whatsup);

    let mut numbers = [1, 2, 3, 4, 5];
    let slice_of_numbers = &mut numbers[1..4];
    slice_of_numbers[0] = 99;
    println!("{:?}", slice_of_numbers);
    */
}
