extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn chapter_three() {

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
    print!("Would you like to skip this function? (1 = yes)\n> ");

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
   print_divider(chapter_two, "chapterTwo");
   print_divider(chapter_three, "chapterThree");
}
