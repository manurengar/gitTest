fn main() {
    // Exercise 1
    let my_age: i8 = 40;
    println!("My age is: {}", my_age);

    // Exercise 2 & 3
    let mut x1: i8 = 40;
    println!("X1 value is: {}", x1);
    let x2: &mut i8 = &mut x1;
    let _even = is_even(x2);
    println!("Now X1 value is: {}", x1);

    // Exercise 4
    let _a: &str = "Three";
    let a: i8 = 10;
    println!("a is {}", a);

    // Exericse 5
    let x: i8;
    x = -1;
    println!("x is {}", x);

    // Exercise 6
    let pi: f64;
    pi = 3.131415;
    println!("Pi is {}", pi);

    // Exercise 7
    let _a1: i8 = -15;
    let _b1: u8 = 170;
    let _name: &str = "Michael";

    // Exercise 8
    type Book = (String, String, u32);
    let _books: Book = (String::from("Book 1"), String::from("Book 2"), 1024);

    // Exercise 9
    let source = &mut 5;
    let multiplication_factor = &mut 3;

    times(add_n(source, 4), add_n(multiplication_factor, 6));
    println!("Source + 4: {} multiplied by 9 = {}", 9, *source);
}

fn is_even(number: &mut i8) -> bool {
    let is_even: bool;
    if *number % 2 == 0 {
        is_even = true;
    } else {
        is_even = false;
    }
    // Fuck this number
    *number = *number / 2;
    return is_even;
}
// See elision lifetime rule
fn add_n(source: &mut i32, n: i32) -> &mut i32 {
    *source += n;
    source
}

fn times<'a>(source: &'a mut i32, times: &'a mut i32) -> &'a mut i32 {
    *source *= *times;
    source
}
