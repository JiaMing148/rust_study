# Rust的高级特性
## 在具有关联类型的特质定义中指定占位符类型
```Rust
trait A {
    //类型Item相当于一个占位符
    type Item;

    fn GetItem(&self) -> Option<Item>;
}

struct B {
    i32 num;
}

impl A for B {
    //由拓展的类型定义Item的类型
    type Item = i32;

    fn GetItem(&self) -> Option<Item> {
        Some(num);
    }
}
```
## 默认泛型类型参数和运算符重载
Rust不允许自己场景操作符或重载任意操作符。但是，可以通过实现与操作符相关的`trait`来重载。    
`std::ops`中列出了相关操作与相关的`trait`。    
示例：
```Rust
use std::ops::Add;

#[derive(Debug)]
pub struct Millimeters(pub u32);
pub struct Meters(pub u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```
## 消除歧义的完全限定语法： 调用同名方法
1. 有参的同名方法调用
```Rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    //当不同特性中拥有同名方法，需要通过指定特性的方式调用。
    Pilot::fly(&person);
}
```
2. 无参的同名方法调用
```Rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    //需要通过指定类型转换 来告诉编译器是指定那个实现的方法
    println!("A baby Animal is called a {}", <Dog as Animal>::baby_name());
}
```
3. 特殊操作： 当两个冲突的trait实现中只有一个被导入时，Rust使用导入的实现。
```Rust
mod inner {
    pub trait A {
        fn f(&self) -> usize { 0 }
    }
    pub trait B {
        fn f(&self) -> usize { 1 }
    }
    pub struct P;
    impl A for P {}
    impl B for P {}
}
fn main() {
    use inner::{P, B};    
    println!("{}", P.f());    
}
```

## Super Trait 要求在一个`trait`在另一个`trait`中的功能
示例：
```Rust
use std::fmt;

fn main() {
    let point = Point {
        x: 3,
        y: 4,
    };
    point.outline_print();
}

//
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        println!("{}", output);
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```
## 使用Newtype模式在外部类型上实现外部特性
