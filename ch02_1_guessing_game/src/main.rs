use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Shhh.... the secret number is {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line, get better at rust fool");

        let guess: u32 = match guess
            .trim()
            .parse()
            {
                Ok(num) => num,
                Err(_) => {
                    println!("Ooo you think you're clever do you?\n\
                    Please type a number next time\n");
                    continue;
                }
            };
           

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Think bigger"),
            Ordering::Greater => println!("Not that big"),
            Ordering::Equal => {
                println!("How did you know!?!?");
                break;
            }
        }

        println!()
    }
}
