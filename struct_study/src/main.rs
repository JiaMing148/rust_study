fn main() {
    let mut user1 = User{
        active:true,
        username:String::from("JiaMing"),
        email:String::from("xxx@foxmail.com"),
        sign_in_count:1,
    };

    user1.email = String::from("JiaMing@foxmail.com");

    let user2 = User{
        email:String::from("MingKo"),
        ..user1 //其余字段的值与user1一致 该操作必须放在最后一行声明
    };
}

//field_init 简写 当字段名与参数名一致时可以免去赋值过程
fn build_user(email: String, username: String) -> User {
    User{
        username,
        email,
        active:true,
        sign_in_count:1,
    }
}

struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}