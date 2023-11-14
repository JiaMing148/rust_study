fn main() {
    // let move_message = Message::Move{x:128, y:2046};
    // move_message.cell();

    let a = 3u8;
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximun is configured to be {}",max),
        _ => (),
    }
    
    if let Some(max) = config_max {
        println!("The maximun is configured to be {}",max);
    }
}

#[derive(Debug)]
enum Message{
    Move{x: i32, y: i32},
    Write(String),
    Quit,
    Color(u8, u8, u8),
}

impl Message{
    fn cell(&self){
        println!("Message is {:?}",self);
    }
}