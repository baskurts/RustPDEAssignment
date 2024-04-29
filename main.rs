fn main(){
    // Introduction to Rust programming language
    println!("Hello World");

    // Variables
    let x = 4;
    println!("x is: {}", x);

    // Data Types
    let floating_point = 10.92;
    println!("{}", floating_point);
    let true_or_false = true;
    println!("{}", true_or_false);
    let letter = ';';
    println!("{}", letter);
    let tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.0);
    let mut arr = [1, 2, 3, 4, 5];
    arr[4] = 3;
    println!("{}", arr[4]);

    // Conditions
    let cond = 2 <= 2;
    println!("{}", cond);

    let food = "bread";

    if food == "cookie" {
        println!("I like cookies too!");
    } else if food == "bread" {
        println!("That sounds great!");
    } else {
        println!("That's bad!");
    }

}

