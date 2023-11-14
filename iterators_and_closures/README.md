# **闭包**
作用：捕获其环境的匿名函数 类似于C#的匿名函数
```Rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    //按照宽度进行排序
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
```

闭包的两种写法
```Rust
//该闭包函数直接借用了局部变量‘s’导致其权限被剥夺
fn a() {
    let mut s = String::from("Hello");
    let mut add_suffix = || s.push_str("world");
}
//该闭包函数相当于声明了一个局部函数，只有在调用时才会借用变量‘s’
fn b() {
    let mut s = String::from("Hello");
    let mut add_suffix = |s: &mut String| s.push_str("world");
}
```

# **迭代器**
特性：迭代器在调用其函数之前，相当于不存在，因为迭代器内部没有变量。

Rust具有零成本抽象，可以大胆的使用Rust中自带的函数，它们会在编译时 编译成优化好的抽象代码