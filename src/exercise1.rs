extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Let's guess the number!");

    print!("How many times would you like to try? [5]: ");

    io::stdout().flush().ok().expect("Could not flush stdout");

    let number = rand::thread_rng().gen_range(1, 100);
    let mut guessing = rand::thread_rng().gen_range(1, 100);

    let mut max = 100;
    let mut min = 0;
    let mut try_times = String::new();
    io::stdin().read_line(&mut try_times).ok().expect("failed to read line.");

    if let Some(mut times) = try_times.trim().parse::<i32>().ok() {
        println!("You have {:?} times.", times);

        loop {

            times -= 1;

            if times <= 0 { break }

            print!("You are guessing: {}, the number is ", guessing);

            match guessing.cmp(&number) {
                Ordering::Greater => {
                    println!("smaller!");
                    if guessing <= max { max = guessing }
                }
                Ordering::Less => {
                    println!("bigger!");
                    if guessing >= min { min = guessing }
                },
                Ordering::Equal => {
                    println!("exactly the same!");
                    break
                },
            }
            guessing = min + ((max - min) as f32 / 2.0).round() as i32;
        }

        if times > 0 {
            println!("You win!")
        } else {
            println!("You lose!");
        }
    } else {
        println!("You have 0 times.");
    }



}
