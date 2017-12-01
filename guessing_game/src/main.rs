extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	let debug = false;
    println!("ИГРА: Угадайте число");

	let secret_number = rand::thread_rng().gen_range(1, 101);
	let mut count = 0;

	if debug {
    	println!("Загаданное число: {}", secret_number);
	}

    loop {
	    println!("Пожалуйста, введите предполагаемое число (от 1 до 100).");

	    let mut guess = String::new(); // изменяемая связь

	    io::stdin().read_line(&mut guess)
	        .expect("Не удалось прочитать строку"); // вызывает panic! с указанным сообщением

	    let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
            	count += 1;
            	num
            },
            Err(_) => {
            	println!("Пожалуйста, введите число!");
            	continue;
            }
        };

	    println!("Ваша попытка: {}", guess);

	    match guess.cmp(&secret_number) {
	        Ordering::Less    => println!("Слишком маленькое!"),
	        Ordering::Greater => println!("Слишком большое!"),
	        Ordering::Equal   => {
	        	println!("Вы угадали за {} попыток!", count);
	        	break;
	        }
	    }
	}
}
