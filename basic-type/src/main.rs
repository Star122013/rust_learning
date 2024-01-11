fn basic_type(){
    let _a: i32 = 1; // i32 (signed) 32bit 默认类型
    let _b: u32 = 1; // u32 (unsigned) 32bit unsigned
    // let d :u8 = 256; // u8 (unsigned) 8bit unsigned 256 超出范围 编译报错 
    let _c: f32 = 1.0; // f32 (float) 32bit 浮点数 单精度
    let _d: f64 = 1.0; // f64 (float) 64bit 浮点数 双精度
    
    // 序列
    for i in 0..10 {  // 0..10 为一个序列不包含10 0..=10 为一个序列包含10
        println!("{}",i);        
    }
    for i in 0..=10{
        println!("{}",i);
    }

    // 字符
    let e: char = 'a'; // char 字符类型
    let f: char = '中'; // char 字符类型 字符用‘’包裹，字符串用“”包裹
    let g: char = '😂'; // char 字符类型
    println!("{} {} {}",e,f,g);
    print!("字符类型占用字节数: {}\n",std::mem::size_of_val(&e)); // std::mem::size_of_val(&e) 获取变量占用字节数

    // bool
    let h: bool = true; // bool 布尔类型
    if h {
        println!("h is true\n");       
    }

    // 单元
    let i: () = (); // 单元类型
    print!("单元类型: {:?}\n",i);
    println!("单元类型占用字节数: {}\n",std::mem::size_of_val(&i)); // std::mem::size_of_val(&i) 获取变量占用字节数

    // 数组
    let j: [i32; 5] = [1,2,3,4,5]; // 数组类型
    println!("数组类型: {:?}",j);
    let _k = [1; 5]; // [1; 5] 为 [1,1,1,1,1] 的简写

    // 元组
    let _l: (i32, f64, char) = (1, 1.0, 'a'); // 元组类型 (i32, f64, char) 为类型 (1, 1.0, 'a') 为值 元组类型可以嵌套 不同类型的元组可以组成元组
    let _m = (1, 1.0, 'a'); // (1, 1.0, 'a') 为 (i32, f64, char) 的简写
    let n = (1, 1.0, 'a', (1, 1.0, 'a')); // 元组类型可以嵌套 不同类型的元组可以组成元组
    println!("元组类型: {:?}",n);

    // 指针
    let o: *const i32 = &1; // 指针类型 不可变指针 用于指向不可变变量 用于指向可变变量会报错
    let p: *mut i32 = &mut 1; // 指针类型 可变指针 用于指向可变变量 用于指向不可变变量会报错
    println!("指针类型: {:?} {:?}",o,p);

    // 表达式
    let y = {
        let x = 1;
        x + 1 // 最后一行不可以加分号
    }; // 表达式类型 表达式的值为最后一行的值 表达式的类型为最后一行的类型 
    println!("表达式类型: {:?}",y);
    let z = if y%2 == 0 {"odd"}else{"even"}; // if 表达式q
    print!("if 表达式: {}\n",z);
}

fn if_else() -> i32 {
    let x = 1;
    if x%2 == 0 {
        println!("even");
    }else{
        println!("odd");
    }
    let y = 1;
    if y%2 == 0 {
        y + 1   
    }else {
        y
    }
}
fn forever() -> ! { // ! 表示函数不会返回 死循环 用于 panic! 宏 
    loop {
      //...
    };
}

fn panic_example() -> ! {
    panic!("panic example");
    
}
fn main(){
    basic_type();
    if_else();
    print!("{}", if_else());
    panic_example();
}