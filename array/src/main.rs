use std::io;
fn main() {
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // Creates an array of 5 3's
    let shorthand_array = [3; 5];

    let _first = shorthand_array[0];
    let _second = shorthand_array[1];

    // Ex:

    let b = [1,2,3,4,5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = b[index];

    println!("The value of the element at index {index} is: {element}");
}
