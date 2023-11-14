fn main() {
    let mut rect1 = Rectangle {
        width: 0,
        height: 0,
    };

    println!("rect1：{:?}", rect1);
    
    println!("rect1 area is {}",rect1.area());

    let other = Rectangle {
        width: 1,
        height: 1,
    };

    rect1.set_width(1);

    println!("maxRect is {:?}",rect1.max(other));

    rect1.set_to_max(other);

    println!("rect area is {}",rect1.area());

    println!("other is {:?}",other);
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width*self.height
    }

    fn set_width(&mut self, width: u32){
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle{
        Rectangle{
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}
