// use an external dependency as outlined in Cargo.toml
extern crate rand;

use std::io; // bring in the io (input/output) library from std (standard library)
use std::cmp::Ordering;
use rand::Rng; // pull Rng from above declared `rand` crate

fn main() {
    const LOWER_BOUND: u32 = 0;
    const UPPER_BOUND: u32 = 50;

    let secret_number: u32 = rand::thread_rng()
        .gen_range(LOWER_BOUND, &UPPER_BOUND + 1);

    println!("Guess the number between {} and {}.", LOWER_BOUND, UPPER_BOUND);

    // creates an infinite loop, exited by using the `break` statement
    loop {
        println!("Enter your guess: ");
        // variables in Rust are inherently inmutable - the use of `mut` in
        // variable declarations allows variables to become mutable ~ to be changed
        let mut guess = String::new();
        // `::` syntax is like a static method of the type, in this case, String.
        // also named "associated function"

        // alternatively could be written as `std::io::stdin()...` if `use std::io`
        // was omitted. `read_line()` reads input from the console and passes the
        // information to a reference (as outlined by the `&`) of the `guess` var
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
            // `.expect()` is needed to suppress Rust warnings. `read_line()`
            // returns a value (specifically of type Result) and if an error
            // occurred, the Result type is used for error handling...

        // 'redeclaration' of guess using `let` keyword again is called shadowing in
        // Rust - allows us to reuse previous variable without creating a new one.
        // shadowing helpful when converting variables from one type to another
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        // `match` is similar to a switch statement, used here to check the returned
        // Result type from the `parse()` method (either `Ok` or `Error`)

        // String placeholders, just like in Python 3
        println!("You guessed: {}", guess);

        // `match` is similar to a switch statement, used here to check the Ordering
        // enum brought in earlier
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small, guess ⬆️"),
            Ordering::Greater => println!("too big, guess ⬇️"),
            Ordering::Equal => {
                println!("You win! 🎉");
                break;
            }
        }
    }
}
