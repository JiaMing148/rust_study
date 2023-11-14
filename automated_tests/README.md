# **编写测试代码**
关键字 #[test]    
1.结构     
```Rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        assert_eq!(10, 10);
    }
}
```
# 特殊语法
测试单一函数
```Rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        assert_eq!(10, 10);
    }

    #[test]
    fn this_test_fail_pass() {
        assert_eq!(10, 8);
    }
}
//$ cargo test this_test_fail_pass
```

测试特定函数
```Rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        assert_eq!(10, 10);
    }

    #[test]
    fn this_test_fail_pass() {
        assert_eq!(10, 8);
    }

     #[test]
    fn other_pass() {
        assert_eq!(8, 8);
    }
}
//前俩名称都包含this 就会执行前俩函数
//$ cargo test this
```

细粒度的限制测试线程
```Rust
//$ cargo test -- --test-threads=1
//限定测试为单线程
```

显示函数输出
```Rust
//cargo test -- --show-output
```

# 忽略测试
关键字 #[ignore]
```Rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        assert_eq!(10, 10);
    }

    #[test]
    #[ignore]
    fn this_test_fail_pass() {
        assert_eq!(10, 8);
    }
}
//$ cargo test -- --ignored
//这样执行就会忽略 this_test_fail_pass
```
# **模块化测试**
在项目中创建tests目录 [tests](./adder/tests)    
tests 目录Rust约定处理过,在该目录声明的脚本无需声明 #[cfg(test)]    
仅在运行cargo test时才会运行tests目录下的脚本    
 
测试指定集成测试函数脚本    
//$ cargo test --test integration_test        
//只运行 integration_test 下的测试函数    