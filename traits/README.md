# **Trait 特征**(类似与 interface)
使用案例：[lib.rs](./src/lib.rs)
# **约束**
```Rust
//普通写法
fn notify(item: &impl Summary) {
     println!("Breaking news! {}",item.summarize());
}

//语法糖写法
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}",item.summarize());
}
//只有拓展了Summary的才可以作为参数传递

//使用+语法约束多个Trait
//普通写法
fn notify(item: &(impl Summary + Display)) {
}
//语法糖写法
fn notify<T: Summary + Display>(item: &T) {
}
//同时拓展了 Summary 与 Display 的才可以作为参数传递
```
注意：使用语法糖的方式约束类型时，传参的具体类型也必须一致，否则需要使用impl的方式。

## 使用where语句 实现约束
```Rust
//当有多个需要约束的参数时
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
}

//使用Where语句
fn some_function<T, U>(t: &T, u: &U) 
where T: Display + Clone, U: Clone + Debug, {
}
```