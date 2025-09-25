fn main() {
    println!("Hello, world!");

    another_function(5,'m');
}

fn another_function(x: i32, y: char){
    println!("Another function.");
    println!("The value of x is: {x}");
    println!("The value of x, then the value of y: {x}{y}")
}
