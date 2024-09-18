fn main()
{
    println!("### nth fibonacci number generator ###\n");
    let input: u32 = input("Please input the Fibonacci number to be generated: ");

    let mut x: u64 = 1;
    let mut y: u64 = 0;
    let mut temp: u64;
    for _counter in 1..input {
        temp = x;
        x += y;
        y = temp;
    }
    println!("the {input}th fibonacci number is: {x}");

    fn input (string: &str) -> u32 {
        loop {
            println!("{string}");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let number = match input.trim().parse::<u32>() {
                Ok(num) => num,
                Err(_) => {continue}
            };
            return number;
        }
    }
}
