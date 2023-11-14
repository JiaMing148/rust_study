fn main() {
    generate_fibonacci_function(10);
}

fn generate_fibonacci_function(count:i32){
    let mut remaining = 0;
    let mut cur_number;
    while remaining <= count{
        remaining += 1;
        cur_number = fib_only_function(remaining);
        println!("the value of n={} value={cur_number}",remaining-1);
    }
}

fn fib_only_function(number:i32) -> i32{
    if number >= 3 {
        fib_only_function(number-1)+fib_only_function(number-2)
    }
    else {
        1
    }
}