use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("数字あてゲームを始めすよ");
    
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop{
        println!("数字を入力してね！");

        let mut guess: String = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("read lineの実行に失敗しました。");
    
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("あなたが推測したのは：{}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("秘密の数字は：{}", secret_number);
                println!("勝利了！");
                break;
            }
        }   
    }
}
