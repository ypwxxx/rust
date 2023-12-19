use rand::Rng;
use std::{cmp::Ordering, io};

//ANCHOR - 一个猜数字游戏

fn main() {
    println!("Guess the number!");

    // 初始化本次猜数字的目标数字
    let target_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        // 生成一个字符串类型的可变变量，存储从外部获取到的输入
        let mut guess: String = String::new();

        // 使用io库提供的方法，获取外部输入
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // 处理输入值为一个正常的数字，如果不是让其反复输入
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guessed: {guess}");

        // 使用match判断输入值的情况，并返回说明，直到输入判断正确
        match guess.cmp(&target_num) {
            Ordering::Less => println!("too small!"),
            Ordering::Equal => {
                println!("you win!!");
                break;
            }
            Ordering::Greater => println!("too big!"),
        }
    }
}
