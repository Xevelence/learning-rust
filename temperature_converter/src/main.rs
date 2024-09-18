

fn main() {
    let mut program_choice: u8;
    println!("### Temperature Converter ###");
    loop {
        let mut input = String::new();
        println!("Please choose on of the following options:\n\
        1: convert from fahrenheit to celsius\n\
        2: convert from celsius to fahrenheit");
        std::io::stdin().read_line(&mut input).expect("Failed to read user input");
        program_choice = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {continue}
        };
        if program_choice == 1 {
            convert_from_fahrenheit_to_celsius(); break;
        }
        else if program_choice == 2 {
            convert_from_celsius_to_fahrenheit(); break;
        }
    }
    fn convert_from_fahrenheit_to_celsius() {
        loop {
            println!("Enter your value to be converted to celsius: ");
            let mut input: String = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read user input");
            let input: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {continue}
            };
            let result: f64 = ((input - 32.0) * 5.0) / 9.0;
            println!("{input}°F equals to {result}°C");
            break;
        }
    }
    fn convert_from_celsius_to_fahrenheit() {
        loop {
            println!("Enter your value to be converted to fahrenheit: ");
            let mut input: String = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read user input");
            let input: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {continue}
            };
            let result: f64 = ((input * 9.0) / 5.0) + 32.0;
            println!("{input}°C equals to {result}°F");
            break;
        }
    }
}
