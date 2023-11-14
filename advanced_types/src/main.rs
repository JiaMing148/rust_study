
fn add_one(num: i32) -> i32 {
    num + 1
}

fn sum_function(f: fn(i32) -> i32, num: i32) -> i32 {
    f(num) + num
}

fn main () {
    println!("sum value is {}",sum_function(add_one, 5));
}

