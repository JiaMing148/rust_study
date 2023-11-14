use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult; //as 可以创建个别名
//use std::{cmp::Ordering, io}; //批量从cmp中引入作用域
//use std::io::{self, Write};// self 代表把io也给引入了

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}