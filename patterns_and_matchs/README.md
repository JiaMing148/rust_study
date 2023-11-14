## `Match` 的使用方式w
1. 普通使用方式
```Rust
enum PAGETYPE {
    AAA(i32),
    BBB(i32),
    CCC(i32),
}

fn Test(t: PAGETYPE) -> i32 {
    match t {
        AAA(i) => i,
        BBB(i) => i,
        CCC(i) => i,
    }
}
```
2. `if let`
```Rust
fn main() {
    let value: Result<u8, _> = "200".parse(); 

    if let Ok(num) = value {
        println!("{}", num);
    }
}
```
### 阴影变量
指代 `match`语句中 枚举(y) 中的变量

### 多个选项
```Rust
let x = 1;

match x {
    1|2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}

//output: one or two
```
### `..=` 数值范围匹配
```Rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}

//output one through five 即1~5.

//ASCII
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _=> println!("something else"),
}

//Rust可以通过ASCII判断数值范围
```
### 解构
1. 解构结构体
```Rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    //将字段值解构
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

2. 解构枚举
```Rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}
```
## `_`的使用
1. 在变量命名时单独使用`_`就是忽略该值
2. 在变量命名时在前方加入`_`就是忽略未使用提示

## `..`省略值的剩余部分
```Rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```
## 附加条件与匹配保护
1. 附加条件
```Rust
let num = Some(4);
match num {
    Some(x) if x % 2 ==0 => println!("The number {} is even", x),
    Some(x) => println!("The number {} is odd", x),
    None => (),
}
//表示为 如果num == Some 并且 x为偶数则进入分支。
```
2. 匹配保护
```Rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _=> println!("no"),
}
//表示为 若x等于 4、5、6 则yes 否则 no.
```