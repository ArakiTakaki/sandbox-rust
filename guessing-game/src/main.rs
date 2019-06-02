// 外部依存先としてrandを指定する
extern crate rand;
use rand::Rng;
// useを使用し、気泡を省略することが可能
// 個人的には省略はあまり使用せずに、そのまま使用してみることにする
//use std::io;

fn main() {
    println!("数を出力してください");
    println!("input num:");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);    //秘密の数字は次の通り: {}
    loop {
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Faild to line"); // エラーハンドリング
        // let gress: u32 = guess.trim().parse()
        //     .expect("hello");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数値を入力してください");
                continue;
            },
        };
        println!("Your grees {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
