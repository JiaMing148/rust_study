use std::env;
use std::process;
use minigrep::Config;

fn main() {
    //env::args() 返回传递给 minigrep 的命令行参数的迭代器
    //args[0] 被程序名占用
    let reuslt = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", reuslt.query);
    println!("In file {}", reuslt.file_path);

    if let Err(e) = minigrep::run(reuslt) {
        println!("Application error: {e}");
        process::exit(1);
    }
}