use std::io;
use std::process::exit;

fn main() {
    println!("Guess the number (input your guess, enter 'e' to exit) !");
    let mut guess = String::new();

    fn doguess(theguess: &mut String) {
        match io::stdin().read_line(theguess) {
            Ok(_) => {
                theguess.pop();
                match theguess.parse::<i64>() {
                    Err(e) => {
                        if theguess == "e" {
                            return
                        }
                        println!("Invalid, please input a number, {}", e);
                    },
                    Ok(number) => if number % 2 == 0 {
                        println!("You guessed wrong !");
                    } else {
                        println!("You guessed right !");
                    }
                }
            },
            Err(_) => println!("A system error happened, please retype !")
        }
        doguess(theguess);
    }
    doguess(&mut guess);
    exit(0);
}
