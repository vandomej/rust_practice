extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

const EXAMPLE_CONST: u64 = 123_456_789;

fn chapter_three() {
    //You can reassign immutable variables
    //by shadowing their value, a.k.a it is
    //the equivalent of creating another immutable
    //variable just with the same name and the 
    //original variable goes out of scope
    let x = 5;
    println!("x: {}", x);

    //original x went out of scope
    let x = x + 2;
    println!("x: {}", x);

    //Tuples are defined inside comma seperated parenthesis.
    //They can be any type (sort of like javascript array), and
    //can be destructured like a javascript object {}
    let tup: (i32, f64, u64) = (-5, 15.206, 200);

    let (x, y, z) = tup;

    println!("y: {}", y);

    //The weird part to look at, is that you can access any component
    //of a tuple using a period after the name, so the equivalent print
    //statement would look like this.
    println!("tup.1: {}", tup.1);

    //Rust arrays are puny and do not resize at all. They are a fixed size
    //and are always on the stack.
    let arr = [-1, 0, 1, 2, 3, 4];
    println!("arr: {:?}", arr);

    //Scopes '{' and '}' are treated as expressions and return something.
    let scope = {
    //Unlike other languages, statements don't actually return a value.
    //So you cant do things like this.
    //let x = (let y = 5);
    };
    //Putting a semicolon at the end of an expression turns it into a statement.
    //Therefore if its a statement, it can no longer return anything.

    println!("scope: {:?}", scope);

    //Now that I think about that, this is the exact same as c and c++...
    //I am pretty far gone.
    fn inline_function() -> i32 {
        println!("In inline function.");
        let return_value: i32 = 32;
        return_value
    }

    println!("before inline...");
    println!("inline function return: {}", inline_function());
    println!("after inline...");

    //if expression
    if x < 0 {
        println!("Wrow, if is right");
    } else {
        println!("Else condition");
    }

    //OKAY GET THIS, IF STATEMENTS ARE FUCKING EXPRESSIONS IN RUST
    //THATS REALLY SMART OF THEM HOLY SHIT.
    let if_result: u64 = if y > 15f64 {
        z
    } else {
        EXAMPLE_CONST
    };
    println!("if statement result: {}", if_result);

    //for loops kind of work like foreach iterators
    let arr = [10, 20, 30, 45, 55, 77];
    
    for element in arr.iter() {
        println!("element value is: {}", element);
    }

    //range forloop
    for num in (1..4).rev() {
        println!("{}", num);
    }
}

fn chapter_two() {
    let secret_number: u32 = rand::thread_rng().gen_range(1, 1001);

    //This block is just me goofing around but
    //if you want to sample some number of random
    //elements from an iterator, you use this
    let mut rng = rand::thread_rng();
    let sample = rand::sample(&mut rng, 500..1000, 10);
    println!("sample is {:?}", sample);
    //End of goofing off

    println!("Welcome to bonus stage!");

    loop {
        println!("Enter your number:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a correct value.");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Your guess was too low"),
            Ordering::Greater => println!("B I G?"),
            Ordering::Equal   => {
                println!("W E L L   D O N E ! ! !");
                break;
            },
        };
    }
}

///What
fn print_divider(delegate: fn(), name: &'static str) {
    println!("==========ABOUT TO ENTER==========");
    println!("{}.", name);
    println!("Would you like to skip this function? (1 = yes)");

    let mut response: String = String::new();

    io::stdin().read_line(&mut response)
        .expect("There was an error reading input from the terminal.");

    let response: u32 = match response.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

    match response.cmp(&1) {
        Ordering::Equal => println!("Skipping {} function", name),
        _ => delegate(),
    }

    println!("==========ABOUT TO LEAVE==========\n\n");
}

fn main() {
    print_divider(chapter_three, "chapterThree");
    print_divider(chapter_two, "chapterTwo");
}
