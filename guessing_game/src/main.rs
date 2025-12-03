use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("猜数游戏");

    let secret_number = rand::rng().random_range(1..=100);

    println!("神秘数字是: {}", secret_number);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("你猜的数是: {}", guess);

    match guess.cmp(&secret_number.to_string()) {
        Ordering::Less => println!("太小了"),
        Ordering::Greater => println!("太大了"),
        Ordering::Equal => println!("恭喜你猜对了"),
    }
}
