// struct 结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 简化结构体实例创建
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {
    // 创建结构体实例
    let mut user1 = User {
        email: String::from("people@example.com"),
        username: String::from("people"),
        active: true,
        sign_in_count: 1,
};
    // 访问修改结构体实例的字段
    user1.email = String::from("people01@example.com");

    let user2 = build_user(String::from("example.com"), String::from("example"));
    println!("{}", user2.email);
    println!("{}", user2.username);
    let user3 = User {
        email: String::from("example.com"),
        ..user2
    };
    println!("{}", user3.active);
    println!("{}", user2.sign_in_count);
    // user2所有权转移给了user3，所以user2不能再使用，但user2的其他字段可以访问
}