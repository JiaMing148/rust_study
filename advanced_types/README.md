# 高级类型
Rust有一个名为 `!` 的特殊类型，在类型理论术语中称为空类型，因为它没有值。我们更喜欢称之为never类型，因为当函数永远不会返回时，它会代替返回类型。

## `fn` 作为参数传递
Rust中 `fn`是一个类型 `||`是一个tirait。
```Rust
fn add_one(num: i32) -> i32 {
    num + 1
}

fn sum_function(f: fn(i32) -> i32, num: i32) -> i32 {
    f(num) + num
}

fn main () {
    println!("sum value is {}",sum_function(add_one, 5));
}
```
## 返回一个闭包
首先，由于闭包函数是一个`trait`，而Rust不知道`trait`的具体内存大小。因此，需要通过`Box<T>`将其包装成`trait对象`才能传递。
```Rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```