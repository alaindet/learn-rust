// This is the **prelude** section
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game");

    loop {
        println!("\nPlease enter a number between 1 and 100: ");

        // gen_range(a, b) INCLUDES a but EXCLUDES b
        // TODO: Snake case enforced?
        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("Picking new random number right now...");

        // // TODO: Remove
        println!("(PSST!, The secret number is {})", secret_number);

        // Declare a new **MUTABLE** variables n.amed "guess",
        // as variables are IMMUTABLE by default
        // "new" is an **ASSOCIATED FUNCTION** of String type, which is
        // similar to a static method in other languages
        let mut guess = String::new();

        // NOTE: references are immutable too, so &mut guess instead of &guess
        // .expect() returns its argument as error message if read_line() returns Err
        // or the read value itself if read_line() succeeded
        // NOTE: Without .expect() you get a compilation warning for unhandled error
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if "exit" == guess.trim() {
            println!("Ok, bye!");
            break;
        }

        // NOTE: This declaration "shadows" the previous one. Useful to cast values
        // The guess is now parsed to an unsigned 32-bit integer (u32)
        // The ": u32" part is an **annotation** suggesting Rust to parse to a u32 type
        // This ignores non-parsable inputs and continues looping
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ERROR: Please provide a valid number");
                continue;
            },
        };

        println!("You picked {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Congratulations, that was {}!", guess);
                break;
            },
        }
    }
}
