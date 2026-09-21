fn main() {
    // Ways of concatenating strings
    // Vectors
    let string_vector: Vec<String> = vec![
        String::from("Nigga,"),
        String::from("what are u"),
        String::from("doing?"),
    ];
    let vector_sentence = string_vector.join(" ");
    println!("{}", vector_sentence);

    // Arrays
    let string_array: [String; 5] = [
        String::from("I"),
        String::from("hate"),
        String::from("Niggers"),
        String::from("&"),
        String::from("Juices"),
    ];
    let array_sentence = string_array.join(" ");
    println!("{}", array_sentence);

    // And without delimiter
    let array_sentence2 = string_array.concat();
    println!("{}", array_sentence2);

    // format! macro
    let part1 = string_array[0..=2].join(" ");
    let part2 = string_array[3..string_array.len()].join(" ");
    let format_sentence = format!("{part1}---{part2}");
    println!("{}", format_sentence);

    // Using a string
    let mut my_string = String::from("Stupid");
    let my_string2 = String::from("Nigger");
    my_string += &my_string2;

    println!("{}", my_string);
    println!("{}", my_string2);
}
