# **smart pointer 智能指针**
1. `Box<T>` 最简单的智能指针   
作用：`Box<T>`允许你将数据储存在堆上而不是在栈上。Box自身没有性能开销，因为它子什么没有什么额外的能力。   
```Rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```
2. `Cons` 使用Box定义递归类型    
```Rust
enum List {
    Cons(i32, List),
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32 ,i32),
}

let list = Cons(1, Cons(2, Cons(3, Nil)));
```

## 使用 `Deref` Trait将智能指针视为常规引用
`Deref` 允许自定义操作符 * (解引用)的行为。   
```Rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
let y = MyBox::new(5);
*y //后台实际运行的是 *(y.deref())
```

## 使用`Drop` Trait清理运行中的代码
`Drop` trait指定当一个值超出作用域时要运行的代码。   
```Rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}
//drop 在被清理时执行的函数 类似析构
```
注意：drop函数不可以主动调用。  

## `RC<T>`引用计数智能指针
```Rust
use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Rc::clone(&a);
    drop(a); //drop 会释放a对这组数据的引用
    drop(b); //这次drop使计数归零，释放数据内存
}
```
`Rc<T>`没有实现DerefMut trait 因此不可解引用（*）

## `RefCell<T>` 和内部可变性模式(Interior mutability)
Interior mutability是Rust中的是一种设计模式，它允许您改变数据，即使存在对该数据的不可变引用。   
注意：该模式使用了`unsafe`代码。  

## Error:引用循环
一个变量同时包含`Rc<T>`&`RefCell<T>`就可能会出现
```Rust
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
```
### `Rc<T>` into a `Weak<T>`为防止出现引用循环的类型
相比于`Rc<T>`只有引用类型归零时才会被释放，`Weak<T>`不需要为0即可被释放。
```Rust
let a = Rc::new(5);
//Rc::downgrade 得到的是Weak<T>类型的智能指针
let b = Rc::downgrade(&a); 
//Weak<T>引用不会增加Rc<T>实例的strong_count计数，而是使用weak_count来统计引用实际的。
println!("a weak_count is {}, strong_count is {} ", a.weak_count(), a.strong_count());
// upgrade 返回类型为 Option<Rc<T>> 当指向的内存存在时 返回Same(T)，反之返回 None。
println!("b value is {}", b.upgrade());
```

## 使用场景总结
1. `Rc<T>`支持同一数据的多个所有者，`Box<T>`和`RefCell<T>`只有一个所有者。
2. `Box<T>`允许在编译时检查不可变或可变借用，`Rc<T>`只允许在编译时检查不可变的借用，`RefCell<T>`允许在运行时检查不可变或可变的借用。
3. `RefCell<T>`允许在运行时检查可变借用，所以即使在`RefCell<T>`不可变的情况下，您也可以改变`RefCell<T>`中的值。