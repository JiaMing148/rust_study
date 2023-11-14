# 宏
术语宏是Rust中的一系列特性，带有`macro_rules!`的声明式宏和三种过程式宏：    
1. 自动有`#[derive]`自定义宏，用于指定结构和枚举使用的`derive`属性添加的代码
2. 类似属性的宏，定义可用于任何项的自定义属性
3. 类似函数的宏，看起来像函数调用，但对指定为其参数的标记进行操作

## 宏和函数的区别
1. 从根本上讲，宏是一种编写代码的方式，这种方式可以编写其他代码，这就是所谓的元编程。在附录C中，我们讨论了 `derive` 属性，它为您生成各种traits的实现。    
2. 元编程对于减少您必须编写和维护的代码量非常有用，这也是函数的作用之一。但是，宏有一些函数没有的额外功能。例如，宏在编译时被拓展，意味着它可以在给定类型上实现trait，而函数是在运行是调用。（trait需要在被编译时实现）

## 带有`macro_rules!`的声明式宏用于通用元编程
```Rust
//这个是 vec!的简化实现，主要用于了解结构。

//注释表明，只要在其中定义宏的crate被带入作用域，该宏就应该可用。如果没有此注释，则无法将宏引入作用域。
#[macro_export]
//宏定义的本名不带!
macro_rules! vec {
    //`$`在宏系统中声明一个变量，该变量将包含于模式匹配的Rust代码。`$`代表它是一个宏变量。
    //`$()`用于捕获于括号内匹配的值，`$x:expr`代表它匹配任何表达式，并将表达式命名为`$x`。
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

## 用于从属性生成代码的过程宏
过程宏的作用更接近于一个函数，通过接受输入，然后对该代码进行操作。     
```Rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenSteam {
}
```

## 如何编写自定义derive宏
[示例(基础trait)](./hello_macro/src/lib.rs)    
[示例(自定义宏)](./hello_macro/hello_macro_derive/src/lib.rs)    
[调用示例](./pancakes/src/main.rs)    

## 类属性宏
类似于自定义宏，但是他们不为`derive`属性生成代码，而是允许创建新属性。     
调用示例：
```Rust
#[route(GET, "/")]
fn index() {
```
宏定义函数的前面如下所示：
```Rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

## 类函数宏
类函数宏定义类似于函数调用，与`macro_rules!`宏类似。
调用示例：
```Rust
let sql = sql!(SELECT * FROM post WHERE id=1);
```
定义示例：
```Rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```
