use core::panic;
use std::io::{self, Write};

fn main() {
    // Exercise 1
    print!("Exercise 1, enter a positive integer (i.e. n=5): ");
    let _ = io::stdout().flush();

    let mut input_number: String = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Error during reading");

    let input_number: i32 = input_number
        .trim()
        .parse()
        .expect("Error when converting number");
    let square_and_sum = square_n_numbers_and_sum(input_number);
    let sum_and_square = sum_n_and_square(input_number);
    let difference = if sum_and_square > square_and_sum {
        sum_and_square - square_and_sum
    } else {
        square_and_sum - sum_and_square
    };
    println!(
        "SquareNSum: {}, SumNSquare:{}, Diff: {}",
        square_and_sum, sum_and_square, difference
    );

    // Exercise 2
    print!("Exercise 2, enter N (i.e. n=20: ");
    let _ = io::stdout().flush();
    let mut input_number: String = String::new();

    io::stdin()
        .read_line(&mut input_number)
        .expect("Error when reading from Terminal");
    let input_number: i32 = input_number
        .trim()
        .parse()
        .expect("Error when converting the number");

    let output_number: i32 = sum_n_numbers(input_number);
    println!("\n The sum result in {output_number}");

    // Exercise 3
    print!("Exercise 3: Number of cars per hour (hours: i.e. 4): ");
    let _ = io::stdout().flush();

    let mut hours: String = String::new();
    let mut speed: String = String::new();

    io::stdin()
        .read_line(&mut hours)
        .expect("Error while reading");
    let hours: i32 = hours.trim().parse().expect("Error while parsing");

    print!("\nSpeed of the factory (from 0 to 10): ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut speed)
        .expect("Error when reading");
    let speed: i32 = speed.trim().parse().expect("Error while parsing");

    println!(
        "\nNumber of cars on {} hours, at speed {} per hour: {}",
        hours,
        speed,
        calculate_number_of_cars(hours, speed, false)
    );
    println!(
        "Number of cars on {} hours, at speed {} per minute: {}",
        hours,
        speed,
        calculate_number_of_cars(hours, speed, true)
    );

    // Exercise 4
}

fn calculate_number_of_cars(time_hours: i32, speed: i32, per_minute: bool) -> i32 {
    if speed < 0 || speed > 10 {
        panic!("Speed provided out of bounds")
    }
    if speed == 0 {
        return 0;
    }

    let faulty_percentage: f64 = match speed {
        1..=4 => 0.0,
        5..=8 => 0.1,
        0..=10 => 0.33,
        _ => 1.0,
    };
    let cars_per_minute_divider = if per_minute { 60 } else { 1 };
    ((time_hours * 221 * speed) as f64 * (1.0 - faulty_percentage) / cars_per_minute_divider as f64)
        .round() as i32
}

fn sum_n_numbers(n: i32) -> i32 {
    let mut counter: i32 = n - 1;
    let mut multiples_vector: Vec<i32> = vec![];
    while counter > 0 {
        if counter % 3 == 0 || counter % 5 == 0 {
            multiples_vector.push(counter);
        }
        counter -= 1;
    }
    multiples_vector.iter().sum()
}

fn square_n_numbers_and_sum(n: i32) -> i32 {
    let mut n_numbers: i32 = 0;

    for i in 0..=n.abs() {
        n_numbers += i.pow(2);
    }
    return n_numbers;
}

fn sum_n_and_square(n: i32) -> i32 {
    let mut n_numbers: i32 = 0;

    for i in 0..=n {
        n_numbers += i;
    }
    n_numbers.pow(2)
}
