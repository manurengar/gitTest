fn main() {
    // Array & type alias
    // Supports slicing
    let mut my_array: [i8; 5] = [0, 1, 2, 3, 4];
    my_array[0] = 33;
    type NewArrayType = [i8; 5];
    let my_new_array: [NewArrayType; 10] = [my_array; 10];
    println!("{:?}", my_new_array);

    // Inclusive slice
    let my_slice1 = &my_new_array[1][0..=3];
    println!("Slice 1: {:?}", my_slice1);

    // Exclusive slice
    let my_slice2 = &my_new_array[2..5];
    println!("Slice 2: {:?}", my_slice2);

    // Vector, mutable arrays
    // Supports slicing
    let mut my_vector: Vec<char> = vec!['C', 'D', 'E', 'F'];
    println!("{:?}", my_vector);
    my_vector[0] = 'X';
    my_vector.push('2');

    let mut my_new_vector: Vec<char> = vec!['0', '2', '3'];
    my_vector.append(&mut my_new_vector);
    println!("vector: {:?}", my_vector);
    println!("new vector: {:?}", my_new_vector);

    // Tuples, arrays that hold any data type
    // NO SLICING, types are differents
    let mut my_tuple = ('A', 21, my_vector, "Hello");
    println!("{}", my_tuple.3);
    my_tuple.1 = 99;
    println!("{:?}", my_tuple);

    return ();
}
