extern crate rand;


use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
	let target = rand::thread_rng().gen_range(1,101);
	println!("Target : {}",target);
	loop{
	println!("Input your number");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("fail to read line");
		let guess : u32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => continue
		};
		println!("you guessed: {}",guess);
		match guess.cmp(&target) {
			Ordering::Less => println!("Too small"),
			Ordering::Greater => println!("Too big"),
			Ordering::Equal => {
				println!("Equal");
				break;
			}
				
		}
	}
}
