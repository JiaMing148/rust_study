# `unsafe` 不安全代码
在不安全代码块中Rust会放开对以下几点的权限:   
1. 取消引用原始指针
2. 调用不安全的函数
3. 访问或修改静态变量
4. 实现不安全的`trait`
5. 访问`union`的字段
## 取消引用原始指针
原始指针与引用和智能指针的不同之处：
1. 通过将不可变指针和可变指针或多个可变指针指向同一个位置，允许忽略借用原则。
2. 不保证指向有效内存。
3. 允许为null。
4. 不实现任何自动清理。
示例：
```Rust
let mut num = 5;
//不可变原始指针
let r1 = &num as *const i32;
//可变原始指针
let r2 = &mut num as *mut i32;
```
### 调用不安全的函数
示例：
```Rust
fn main() {
    unsafe {
        dangerous();
    }
}
unsafe fn dangerous() {}
```
不安全的函数需要放在不安全代码块中运行。
## 使用 `extern` 函数调用外部代码
```Rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
//在 extern "C"块中，我们列出了我们想要调用的另一种语言的外部函数的名称和签名。
//其中"C"中，定义外部函数使用哪个应用程序二进制接口（ABI）。
```
在Rust中使用其它语言的外部函数总是不安全的，因为它们可能不符合Rust的检查标准。   
    
从其他语言中调用Rust函数的示例：   
```Rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```
### 访问或修改可变静态变量
```Rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
//读写可变静态变量是不安全的
```
## 实现不安全的特性
```Rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

//通过使用 `unsafe impl` ，我们承诺我们将维护编译器无法验证的不变量。
```
//Rust中如果一个类型全部由`Send`和`Sync`的类型组成，那么编译器会自动将该类型归为`Send`与`Sync`。
//但是如果其中包含了原始指针等，并且想要将该类型标记为`Send`或`Sync`，则必须使用`unsafe`。
## 访问联合的字段
最后一个仅适用于 `unsafe` 的操作是访问联合的字段。 `union` 类似于 `struct` ，但在特定实例中一次只使用一个声明的字段。联合体主要用于与C代码中的联合体进行交互。访问`union`字段是不安全的，因为Rust不能保证当前存储在`union`实例中的数据的类型。详情参考[了解更多关于`union`的信息](https://doc.rust-lang.org/reference/items/unions.html)。