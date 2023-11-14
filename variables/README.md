# **variables**
注意：Rust中声明与分配是分开的，当单独声明 let x; 时并不会分配任何内存。  

# **DataType 数据类型 栈数据**
## 整数类型
```Rust
//细分类型： i(有负数) u(无负数)  
i8 i16 i32 i64 isize u8 u16 u32 u64 usize //(size 基于允许环境CPU的架构)
```
i的赋值区间:   
```
[-(2^n-1), (2^n-1)-1] 
n = bit
```
u的赋值区间:   
```
[0, (2^n-1)-1]
n = bit
```
特殊语法： 数字允许使用 '_' 例如：1_000 来表示 1000
```Rust
let a:i32 = 1000;
let b:i32 = 1_000;
// a == b 
```
注意事项：当在发布模式(Release Mode)下运行时,如果数值超出赋值区间那么该数值会从头开始。  

## 浮点类型
```Rust
f32 f64

//隐式声明 
let a = 2.0; //f64

//显式声明 
let a:f32 = 3.0; //f32
let a:f64 = 3.0; //f64
```
## bool 布尔类型
bool : true  false  

## Char 字符类型
```Rust
//隐式声明：
let c = 'z';
//显式声明：
let c:char = 'z';
```
### 技巧
```Rust
let a = 3u8 //可以指定声明对应的数据类型
```
# **复合类型**
## 元组 tuple
```Rust 
//隐式声明：
let tup1 = (648,255,1.0);
//显式声明：
let tup2:(i32,u8,f64) = (648,255,1.0);
//元组解构：
let (x,y,z) = tup2; //批量声明了三个变量 一一对应
//索引访问：
let x = tup2.0; // 访问的第一个item

//允许混合类型存储
```

## 数组
```Rust
//隐式声明：
let arr1 = [1,2,3,4,5,6,7];
//显式声明：
let arr2 [i32; 5] = [1,2,3,4,5];
//初始化声明： 
let arr3 = [3; 5]; //相当于声明 数据长度为5 item都是3的数组。
//索引访问: 
let i = arr[0];

//特点：开辟在栈中.  
```

# **Functions 函数**
关键字: "fn"  
命名规范: 函数名_function  
传参格式: function(x:i32)  
特殊：语句块 { let x=5; x+1 } 结尾的语句不加; 就是返回值 例子中返回 6  
带返回值的函数声明: function()-> i32 { }   

# **分支 判断**
## if 表达式
关键字 if 、 else 、else if  
特殊：判断条件不加括号与lua相同  
特殊用法：let number = if condition { 5 } else { 6 } 类似三目运算符  
## 循环
关键字 loop、while、for  
## 关键字 loop
死循环 通过 break; 跳出 continue 跳过一次  
循环标签 'loop_tag: loop{}  
使用 break 'loop_tag 可以从其它内循环中结束该循环  

## 关键字 while 
声明方式： while 条件 { 循环体 }   

## 关键字 for
声明方式： for element in arr {} 类似于C# foreach  
for实现方式：基于标准库中的Range,它按顺序生成从一个数字开始到另一个数字之前结束的所有数字  
