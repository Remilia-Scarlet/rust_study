use std::io;

fn main() {
    println!("Guess the number");
	println!("Input your number");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess).expect("fail to read line");
	println!("you guessed: {}",guess)
}
