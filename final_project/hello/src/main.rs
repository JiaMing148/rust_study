use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use hello::ThreadPool;

fn main() {
    //TcpListener 可以监听地址的Tcp链接
    //bind 返回 Result<T, E> 在这个使用场景中，bind的作用与new的作用类似。
    //TODO 低于1023的端口需要管理员权限才能访问。
    let listener = TcpListener::bind("192.168.18.172:7878").unwrap();
    //线程池 限制了4个线程
    let pool = ThreadPool::build(4);

    //incoming返回一个迭代器
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

        println!("Shutting down.");
    }
}
//处理连接 阅读请求
fn handle_connection(mut stream: TcpStream) {
    //BufReader实例，它包装了一个对`stream`的可变引用
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let response = get_response(&request_line);
    stream.write_all(response.as_bytes()).unwrap();
}

fn get_response(line: &str) -> String {
    let (status_line, filename) = match &line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _=> ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    //fs 标准库的文件系统模块 阅读项目读取文件内容是使用
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    //Content-Length 它被设置为我们的响应体的大小
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}