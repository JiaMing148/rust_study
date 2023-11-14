fn main() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];

    println!("{hello}_{world}");

    let s2 = "HELLO WORLD";
    let hello2 = &s2[..5];
    let world2 = &s2[6..];

    println!("{hello2}_{world2}");

    let a = [1,2,3,4,5,6];
    let slice = &a[..3];

    for item in slice {
        println!("itemValue is {}",item);
    }
}