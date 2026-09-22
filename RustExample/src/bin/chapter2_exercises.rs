use std::io::{self, Write};

fn main() {
    // Exercise 1
    print!("Enter number to calculate EX1 difference: ");
    io::stdout().flush();

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
    println!(
        "SquareNSum: {}, SumNSquare:{}, Diff: {}",
        square_and_sum,
        sum_and_square,
        square_and_sum - sum_and_square
    );
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
        n_numbers += n;
    }
    n_numbers.pow(2)
}
