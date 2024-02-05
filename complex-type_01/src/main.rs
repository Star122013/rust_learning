// string
fn greet(name :&String){
    println!("hi {}", *name);
}

// 切片 slice
fn example_slice() {
    let s = String::from("Hello world!");
    let hello = &s[0..5];
    let world = &s[5..];
    println!("{} {}", hello, world);
}

// 字符串追加,插入
fn example_string_append() {
    let mut s = String::from("Hello");
    s.push_str("world!");
    s.push('!');
    s.insert(5,',');
    s.insert_str(0, "I say ");
    println!("{}", s);
}
// 字符串替换
fn example_string_replace() {
    let mut s = String::from("Hello world!\n");
    let s1 = s.replace("world", "rust");  // replace 返回新的字符串，原字符串不变 适用于String &str
    let s2 = s.replacen("Hello", "Hi", 1); // replacen 返回新的字符串，原字符串不变 适用于String &str
    s.replace_range(0..5, "Hi"); // replace_range 原字符串改变 适用于String
    println!("{} {} {}", s,s1,s2);
}

// 字符串删除
fn example_string_remove() {
    let mut s = String::from("Hello world!\n");
    let s1 = s.pop();
    s.remove(0); // remove 删除指定索引的字符 原字符串改变 适用于String
    s.drain(0..5); // drain 删除指定索引范围的字符 原字符串改变 适用于String
    println!("{}", s);
    dbg!(s1);
}

// 元组 tuple
fn example_tuple() {
    let tuple01: (i32, i32, i32, i32, i32) = (1, 2, 3, 4, 5);
    let (a, b, c, d, e) = tuple01;
    println!("{} {} {} {} {}", a, b, c, d, e);
    let (a, b, _, _, e) = tuple01;
    println!("{} {} {}", a, b, e);
    let tuple02: (i32, i32, i32, i32, i32) = (1, 2, 3, 4, 5);
    println!("{}", tuple02.0);
}

fn calaulate_length(s: &String) -> (&String, usize){
    let length = s.len();
    (s, length)
}




fn main(){
    let s = String::from("xiaoming");
    // let s = "xiaoming";  // &str error 
    greet(&s);
    example_slice();
    let s1 = &s;
    println!("{}", *s1);
    example_string_append();
    example_string_replace();
    example_string_remove();
    example_tuple();
    let (s, length) = calaulate_length(&s);
    print!("{} {}", s, length);
}

