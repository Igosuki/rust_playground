use std::io;
use std::process::exit;
use rand::Rng;

fn main() {
    println!("Guess the number (input your guess, enter 'e' to exit) !");
    let mut guess = String::new();
    let magic_number = rand::thread_rng().gen_range(0, 101);
    doguess(&mut guess, magic_number);
    exit(0);
}

fn doguess(theguess: &mut String, magic_number: i64) {
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
                Ok(number) => if number == magic_number {
                    println!("You guessed right !");
                    return
                } else if number > magic_number {
                    println!("You guess is too high !");
                } else {
                    println!("You guess is too low !");
                }
            }
        },
        Err(_) => println!("A system error happened, please retype !")
    }
    theguess.clear();
    doguess(theguess, magic_number);
}
