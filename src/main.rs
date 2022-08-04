
use std::{cmp::Ordering, io, io::Error, io::ErrorKind};
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
     pub fn new(value: i32) -> Result<Guess, Error> {
        if value < 1 || value > 100 {
           return Err(Error::new(ErrorKind::Other, "Guess value must be positive integer number between 1 and 100."));
        }
        Ok(Guess { value })
     }
     // we use getter so that value can reamin private and there for can not be set explicitly, setting it explycitly with out the new() function would bypass validation which we dont want to allow
     pub fn value(&self) -> i32 {
         self.value
     }
}

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");//cheat code..!!


   loop {
      println!("Please input your guess.");

      let mut guess = String::new();

      io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read line");

      let guess: Guess = match guess.trim().parse() {
           Ok(num) => match Guess::new(num) {
                   Ok(value) => value,
                   Err(err) => {
                        println!("{}", err);
                        continue; 
                   } 
           }
           Err(e) => {
               println!("letter or random characters are not valid in the game lead to: {}", e);
               continue;
           }
      };
  /* //implementing struct with method to do not repeat it ant other circuntances that may whould be needed
      if guess < 1 || guess  > 100 {
         println!("The secret number will be between 1 and 100.");
         continue;
      } 
  */
      println!("You guessed: {}", guess.value());

      match guess.value().cmp(&secret_number) {
           Ordering::Less => println!("Too small!"),
           Ordering::Greater => println!("Too big!"),
           Ordering::Equal => {
               println!("You win!");
               break;
           }
                 
      }
   }
  
    
}

/*la vercion inicial no tenia robustes para enfrentar a los casos de errores en adversos esenarios
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    fn main() {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
*/