use std::{cmp::Ordering, io::stdin};

use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Угадай число");
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .ok()
            .expect("Не удалось прочитать строку");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("Меньше"),
            Ordering::Greater => println!("Больше"),
            Ordering::Equal => {
                println!("Выиграли {}", guess);
                return;
            }
        }
    }
}
