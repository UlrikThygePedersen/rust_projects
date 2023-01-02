use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    let num1: i32 = args[2].parse().unwrap();
    let num2: i32 = args[3].parse().unwrap();

    if args[1] == "add" {
        println!("The sum of {} and {} is {}!", num1, num2, num1 + num2);
    }

    if args[1] == "sub" {
        println!(
            "The difference between {} and {} is {}!",
            num1,
            num2,
            num1 - num2
        );
    }

    if args[1] == "prod" {
        println!("The product of {} and {} is {}!", num1, num2, num1 * num2);
    }

    if args[1] == "divide" {
        println!("The division of {} and {} is {}!", num1, num2, num1 / num2);
    }

    if args[1] == "remain" {
        println!("The division of {} and {} is {}!", num1, num2, num1 % num2);
    }
}
