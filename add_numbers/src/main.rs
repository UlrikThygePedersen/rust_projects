use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    let num1: i32 = args[1].parse().unwrap();
    let num2: i32 = args[2].parse().unwrap();

    println!("The sum of {} and {} is {}!", num1, num2, num1 + num2);

    print({}, args)
}
