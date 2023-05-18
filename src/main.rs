use std::io;

mod calc;

fn main() {
    println!("Hello, this is a calculator!");

    let mut x = String::new();
    let mut y = String::new();
    let mut z = String::new();
    let ans: i32;

    println!("Input the operation:\n 1 for addition\n 2 for subtraction\n 3 for multiplication\n 4 for division\n");
    io::stdin().read_line(&mut z).expect("Failed to read line");

    let z: usize = z.trim().parse().expect("number was not included");

    println!("Input the first number:");
    io::stdin().read_line(&mut x).expect("Failed to read");

    let x: usize = x.trim().parse().expect("number was not included");

    println!("Input the second number:");
    io::stdin().read_line(&mut y).expect("Failed to read");

    let y: usize = y.trim().parse().expect("number was not included");

    if z == 1 {
        ans = calc::add(x.try_into().unwrap(), y.try_into().unwrap());
        println!("The answer is {}", ans)
    } else if z == 2 {
        ans = calc::sub(x.try_into().unwrap(), y.try_into().unwrap());
        println!("The answer is {}.", ans)
    } else if z == 3 {
        ans = calc::mul(x.try_into().unwrap(), y.try_into().unwrap());
        println!("The answer is {}.", ans)
    } else if z == 4 {
        ans = calc::div(x.try_into().unwrap(), y.try_into().unwrap());
        println!("The answer is {}.", ans)
    }
}
