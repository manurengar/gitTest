use rand::Rng;

fn main() {
    // Review of the differnt loops
    // Simple loop
    let mut counter: i32 = 1;
    loop {
        counter += 1;
        println!("{}", counter);
        if counter >= 10 {
            break;
        }
    }

    let second_counter = loop {
        let counter = 1;
        break counter * rand::thread_rng().gen_range(1..=15);
    };
    println!("Random number: {}", second_counter);

    // Naming loops
    'outer: loop {
        'inner: loop {
            println!("Fuck NIggers");
            break 'outer;
        }
    }

    // For loops
    let mut my_vector: Vec<&str> = vec![
        "Hi nigga",
        "how are you?",
        "stupid jew",
        "u are all controlled",
    ];
    // Working with index
    for i in 0..my_vector.len() {
        println!("Index: {i}, value: {} ", my_vector[i]);
    }

    // Another way to obtain and work with the index
    let vector_enumeration = my_vector.iter().enumerate();
    for (index, item) in vector_enumeration {
        println!("Enumeration: {index}, item: {item}");
    }

    // Without index // This in destroys the vector ('moves it')
    for item in my_vector {
        println!("{}", item);
    }

    // While loop - basic counter
    let mut counter = 1;
    while counter <= 10 {
        println!("Counter {counter}");
        counter += 1;
    }

    // Idiomatic while, most used when poping elements of a collection
    let mut my_vector: Vec<u32> = vec![12, 31, 41, 51, 6];

    while let Some(my_val) = my_vector.pop() {
        println!("Pop element: {my_val}");
    }
}
