// RALK > MAIN.RS

use std::env;
use rand; // for RNG functionality

fn main() {
    // create new variable 'args'
    // type: Vector (basically an array)
    // env::args().collect will "collect"
    // all of the arguments and put them in
    // a nice vector (array) for us to use <3
    // see also: https://doc.rust-lang.org/std/env/fn.args.html
    let args: Vec<String> = env::args().collect();

    // this statement lets Rust infer the type of
    // the variable. in this case, `operator`.
    let operator = &args[1];

    // return a random number of type u64
    if operator == "r" {
        println!("{}", rand::random::<u64>());
        std::process::exit(0);
    }
    // omitted &, as var references cannot be converted
    let num1: i64 = args[2].parse().unwrap();
    let num2: i64 = args[3].parse().unwrap();

    if operator == "a" {
        let sum: i64 = num1 + num2;
        println!("your sum is: {sum}");
    } else if operator == "s" {
        let sum: i64 = num1 - num2;
        println!("your sum is: {sum}");
    } else if operator == "d" {
        let sum: i64 = num1 / num2;
        println!("your sum is {sum}");
    } else if operator == "m" {
        let sum: i64 = num1 * num2;
        println!("your sum is {sum}");
    } else {
        println!(">~< operator {operator} unrecognized!!");
    }
}
