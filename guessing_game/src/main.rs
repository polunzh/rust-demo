use rand::Rng;

use std::cmp::Ordering;
use std::io;
fn main() {
  println!("Guessing game!");

  let secret_num = rand::thread_rng().gen_range(1, 101);
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
    println!("You guessed: {}", guess);
    match guess.cmp(&secret_num) {
      Ordering::Greater => println!("Too big"),
      Ordering::Less => println!("Too small"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
