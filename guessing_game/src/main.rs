use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess: String = String::new();
        println!("Enter a number between 1 and 100: ");
        std::io::stdin().read_line(&mut guess).expect("failed to read input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You entered: {guess}");

        if guess > random_number { println!("Your guess is too high"); }
        else if guess < random_number { println!("Your guess is too low"); }
        else { println!("You won!"); break; }
    }
}
