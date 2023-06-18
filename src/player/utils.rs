use std::io;

pub(super) fn read_usize(min_value: usize, max_value: usize) -> usize {
    loop {
        println!(
            "Please enter an integer between {} and {}:",
            min_value, max_value
        );

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<usize>() {
                Ok(number) => {
                    if number >= min_value && number <= max_value {
                        return number;
                    } else {
                        println!("Input out of range. Please try again.");
                    }
                }
                Err(_) => {
                    println!("Invalid input. Please try again.");
                }
            },
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}
