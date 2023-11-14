# **Enum 枚举**
声明方式:  
1. 
```Rust
enum IpAddress{
    v4,
    v6
}  
let four = IpAddress::v4;  
```

2. 
```Rust
enum IpAddress{  
    v4(String),  
    v6(String)
}
let four = IpAddress::v4(String::from("127.0.0.1"));
```

3. 
```Rust
enum IpAddress{
    v4(u8,u8,u8,u8),
    v6(String)
}
let four = IpAddress::v4(127,0,0,1);
```

特点:枚举可以声明拓展块与结构体一样声明方法。  

总结:枚举中的每个枚举的关联值可以是任何类型。  

Rust 用于替换Null的方式是使用一个名为Option的Enum。  
```Rust
//结构
enum Option<T>{
    None,
    Some(T),
}
```

# **match 关键字** (类似于C#的switch)  
使用方式：  
```Rust
match ip {
    IpAddress::v4 => 1,
    IpAddress::v6 => 2,
}
```