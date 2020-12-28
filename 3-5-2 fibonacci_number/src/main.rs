use std::io;

fn main() {
    println!("Example #3-5-2. Getting a n-th Fibonacci Number");

    println!("Input an ordinal number of Fibonacci Sequence to get a correspond element(0-92):");

    let mut ordinal_number = String::new();
    io::stdin().read_line(&mut ordinal_number)
        .expect("Failed to read line.");
    
    let ordinal_number: u8 = match ordinal_number.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("The value must be numeric, got {}", ordinal_number),
    };

    println!("Select method(1-3):
    1. Most simple, most slow
    2. WHILE
    3. FOR");

    let mut method = String::new();
    io::stdin().read_line(&mut method)
        .expect("Failed to read line.");
    
    let method: u8 = match method.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("The value must be numeric, got {}", method),
    };

    match method {
        1 => println!("The result is {}.", get_fibonacci_1(ordinal_number)),
        2 => get_fibonacci_2(ordinal_number),
        3 => get_fibonacci_3(ordinal_number),
        _ => println!("Not a valid method."),
    }
}

fn get_fibonacci_1(x: u8) -> u64 {
    match x {
        0 => 0,
        1 => 1,
        _ => get_fibonacci_1(x - 1) + get_fibonacci_1(x - 2),
    }
}

fn get_fibonacci_2(x: u8) {
    let mut a: u64 = 0;
    let mut b = 1;
    let mut i = 0;
    while i < x {
        let c = b + a;
        a = b;
        b = c;
        i = i + 1;
    }
    println!("The result is {}.", a);
}

fn get_fibonacci_3(x: u8) {
    let mut a: u64 = 0;
    let mut b= 1;

    for _ in 0..x {
        let c = b + a;
        a = b;
        b = c;
    }
    println!("The result is {}.", a);
}