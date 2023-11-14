# 设计权衡测试题 盲点总结
1. `Rc<T>`是不可变指针,`RefCell<T>`可变指针
2. `type`内联类型，在trait中
```Rust
trait MoveType {
    type Move： MoveType; //随着实现的结构体而转变类型
}
```
3. `dyn` 表示动态特性类型

