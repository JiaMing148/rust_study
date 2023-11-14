use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数字！");

    let secret_number = rand::thread_rng()
                        .gen_range(1..=100);

    loop{
        println!("请输入你的答案。");

        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败了");

        //关键字 match 
        //parse 会返回一个Result的枚举 其中OK代表成功 Err代表异常 expect函数在Err时会执行该函数。
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,  //忽略输入文字时的保存 continue 跳过本次循环
        };
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("猜对了!");
                break;           //跳出本次循环
            },
        }
    }
}
