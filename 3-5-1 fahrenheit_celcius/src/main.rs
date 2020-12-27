use std::io;

fn main() {
    println!("Example #3-5-1. Bidirectional conversion between Fahrenheit and Celsius");

    loop {
        println!("Available actions:
        1. Fahrenheit to Celcius
        2. Celcius to Fahrenheit
        3. Quit");
        println!("Choose your action:");

        let mut action = String::new();
        io::stdin().read_line(&mut action)
            .expect("Failed to read line.");

        let action: i32 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
    
        if action > 3 || action == 0 {
            println!("Provide a valid number of action.");
        } else if action == 3 {
            println!("Exiting...");
            break;
        } else if action == 1 || action == 2 {
            println!("Fahrenheit to Celcius - Enter the Fahrenheit degree...");

            let mut input_value = String::new();
            io::stdin().read_line(&mut input_value)
                .expect("Failed to read line.");

            convert_degree(input_value, action);
        }
    }
}

fn convert_degree(input_value: String, action: i32) {
    let input_value: f64 = match input_value.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("The value must be numeric, got {}", input_value.trim()),
    };

    if action == 1 {
        let result = (input_value - 32.0) * 5.0/9.0;

        println!("You entered {}°F, now converted to {}°C.", input_value, result);
    } else if action == 2 {
        let result = (input_value * 9.0/5.0) + 32.0;
    
        println!("You entered {}°C, now converted to {}°F.", input_value, result);
    } else {
        println!("Action number {} is not valid.", input_value);
    }
}