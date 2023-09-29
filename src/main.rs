
use std::io; // the 'use' statement imports the input/output library from the RUST standard library
use std::cmp::Ordering; // imports the the Ordering enum from the standard library
use rand::Rng; // imports the rng library from the rand crate
use std::thread::sleep;
use std::time::Duration;
fn main() { // the entry point of the program

    println!("\nGuess the number between 1 - 100 in 10 attempts, use [Ctrl + C] to exit");
    println!("-------------------------------------------------------------------\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 10;

    loop {

        if attempts == 0 {
            println!("You did not figure out the correct answear\nTHE ANSWEAR WAS: {secret_number}");
            sleep(Duration::from_secs(5));  // used as a bandaid to stop the console instantly closing when running the exe directly
            break;
        }

        if attempts == 1{

            println!("Please input your guess. You have {attempts} attempt remaining");

        } else {
            
            println!("Please input your guess. You have {attempts} attempts remaining");

        }


        let mut guess = String::new(); // the let statement creates a variable, in this case we have created a string value called guess. (the mut tage makes it a mutable value, aka can be modified later on in the script)

        io::stdin() // This line calls the stdin function from the IO lib, this allows us to handle user input 
            .read_line(&mut guess) // the first method called is the read_line method. it takes the input and writes it to the guess value
            .expect("Failed to read line"); // Helps provide context for errors (not the exact function, but works for me :P)
            

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nYou guessed: {guess}");

        match guess.cmp(&secret_number) {

            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                attempts -= 1;
                println!("🏆 YOU WIN!! with {attempts} attemps remaining");
                sleep(Duration::from_secs(5)); // used as a bandaid to stop the console instantly closing when running the exe directly
                break;} // ends loop if the correct number is gussed

        }

        println!("\n-------------------------------------------------------------------\n");

        attempts -= 1;

    }

}
