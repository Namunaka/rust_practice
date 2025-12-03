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

    let guess: u32 = guess.trim().parse().expect("请输入一个数字");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("太小了"),
        Ordering::Greater => println!("太大了"),
        Ordering::Equal => println!("恭喜你猜对了"),
    }
}
