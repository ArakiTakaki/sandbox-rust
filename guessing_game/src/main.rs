extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数を入力してくだささい");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("please input gress ");
        let mut grees = String::new();

        io::stdin().read_line(&mut grees).expect("input error");

        println!("Your greees {}", grees);

        // expectのエラーハンドリングはmatchでも実装可能
        let grees: u32 = match grees.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match grees.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
