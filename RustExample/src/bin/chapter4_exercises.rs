use std::vec;

fn main() {
    // Exercise 1
    let s1: String = String::from("Nigger");
    let s2: &str = "Stupid nigger";
    exercise1(&s1, s2);
    println!("Again, a:{}, b:{}", s1, s2);

    // Exercise 2
    let mut my_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut my_vec_2: Vec<i32> = vec![];
    while !my_vec.is_empty() {
        println!("Elements on the temporal array are: {:?}", my_vec_2);
        if let Some(popped_element) = my_vec.pop() {
            my_vec_2.push(popped_element);
            println!("Popped element: {}", popped_element);
        }
    }

    // Exercise 3
    let str1 = exercise3();
    {
        let str2 = str1;
        println!("Exercise 3: {}", str2);
    }

    // Exercise 4
    let mut some_vector = vec![1, 2, 3];
    let first_element = exercise4(&some_vector);
    some_vector.push(4);
    println!("The first element is: {}", first_element);

    // Exercise 5
    let vec_ex_1 = vec![1, 2, 3];
    let vec_ex_2 = vec![4, 5, 6];
    let mut vec_pointer: &Vec<i32>;
    vec_pointer = &vec_ex_1;
    println!("Vector pointer is pointing to vec1 {:?}", vec_pointer);
    vec_pointer = &vec_ex_2;
    println!("Vector pointer is pointing to vec2 {:?}", vec_pointer);

    //Exercise 6
    let mut first_num = 42;
    let mut second_num = 64;
    let ref1 = &mut first_num;
    let mut ref2 = &mut second_num;
    *ref1 = 15;
    *ref2 = 10;
    ref2 = ref1;
    println!("Current pointer 2 towards: {}", ref2);
}

fn exercise4(vec: &Vec<i32>) -> i32 {
    vec[0]
}

fn exercise3() -> String {
    String::from("Stupid nigger")
}

fn exercise1(a: &String, b: &str) {
    println!("a: {}, b: {}", a, b);
}
