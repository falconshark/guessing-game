use std::io;

fn main() {
    println!("数字あてゲームを始めすよ");
    println!("数字を入力してね！");

    let mut guess: String = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("read lineの実行に失敗しました。");

    println!("あなたが推測したのは：{}", guess);
}
