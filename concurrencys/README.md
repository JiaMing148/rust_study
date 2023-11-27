# 无畏的并发
## 使用线程并行代码
可能导致的问题：   
1. 争用条件，其中线程以不一致的顺序访问资源或数据。
2. 死锁，两个线程相互等待，导致俩线程停滞。
3. 其它罕见的问题。    
### 使用`spawn`创建新线程
```Rust
use std::thread;
use std::time::Duration;
//注意：线程会在主线程终止时停止,线程间的运行是无序的。
thread::spawn(|| {
    for i in 1..10 {
        println!("in spawned thead {}", i);
        //sleep 使线程停止运行一段时间
        thread::sleep(Duration::from_millis(1));
    }
});
```
### 使用 `join`Handles 等待所有线程完成
作用：会阻塞其它运行中的线程，直到`join` Handle运行完毕。

### 使用`move`闭包和线程
作用：可以强制使闭包获取它所使用的值。

## 线程间传递数据 以消息的方式
```Rust
use std::sync::mpsc;
use std::thread;

fn main() {
    //mpsc::channel 返回发送器与接收器
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        //val被发送后该线程内的val将被不存在
        tx.send(val).unwrap();
    });
    //rx 内置两种获取函数 recv & try_recv
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```
**注意事项**：   
1. 发送器可以通过`clone`函数创建多个，接收器只能存在一个。
2. 普通的发送器只能发送单一类型的值。

## 共享状态并发
### Mutex (mutual exclusion) 互斥锁
作用： 在任何给定的时间内只允许一个线程访问某些数据。     
使用原则：
1. 必须在使用数据之前尝试获取锁`lock`。
2. 当您处理完互斥锁保护的数据时，必须解锁数据，以便其它线程可以获取锁。   
#### 使用`Arc<T>`进行原子引用计数
`Arc<T>`是一个类似于`Rc<T>`的类型，可以安全的在并发情况下使用。 a代表原子，它是原子引用计数类型。  
关于原子详情见*库：`std::sync::atomic`*。    

## `Sync`&`Send` traits的可拓展并发性
`std::marker`中的两个并发概念：`Sync`&`Send`
### `Send` traits
作用：实现 `Send` traits 类型的值 允许所用权在线程之间转移。    
Rust中几乎所有的原始类型都是 `Send` ，除了原始指针。  
### `Sync` traits
作用：实现 `Sync` traits 类型的值 允许并发访问。

### 总结
1. 手动实现 `Sync`&`Send` traits 不安全。
因为有 `Sync`&`Send` traits组成的类型也会自动成为 `Sync`&`Send`.   
2. Rust处理并发的方式大多以creta的形式，线上的crate比标准库发展的更加先进。  
3. Rust标准库提供了 `Mutex<T>` 和 `Arc<T>`,他们可以在并发情况下安全的使用。

## mpsc模块 
作用：允许我们在线程之间传递消息
```rust
use std::sync::mpsc
use std:thread

fn main(){
  let (tx, rx) = mpsc::channel();
  let tx2 = mpsc::Sender::clone(tx);

  thread::spawn(move|| {
     let s = String::from("AAA");
     tx.send(s).unwrap();
  });

  thread::spawn(move|| {
     let s = String::from("BBB");
     tx2.send(s).unwrap();
  });

  for item in rx {
     println!("{}", item);
  }
}
```
