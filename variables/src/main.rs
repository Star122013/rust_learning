struct Struct{ // struct is a complex variable, it can be used to store multiple values in one variable struct是一个复杂的变量，它可以用来在一个变量中存储多个值
    e: i32
}
fn main(){
    let x = 5; // let is used to declare a variable let用来声明一个变量 i32 is a signed 32-bit integer i32是一个有符号的32位整数
    // x=6 this will throw an error, because x is immutable 这会扔出来一个错误，因为x是不可变的
    print!("the value of x is {}\n", x);

    let mut y = 5;  // mut means mutable mut意味着可变,此时就不会在报错
    print!("the value of y is {}\n", y);
    y = 6;
    print!("the value of y is {}\n", y);

    let _a = 1; // use _ to avoid unused variable warning 使用_来避免未使用变量的警告
    // let a = 2; // this will throw an error, a is not used 这会扔出来一个错误，因为a没有被使用

    let (c, mut b): (bool, bool) = (true,false); // let 可以进行变量的解构，从复杂变量中提取出来 let can destructure variables, extract from complex variables
    print!("the value of c is {}\n", c);
    print!("the value of b is {}\n", b);
    b = true;
    print!("the value of b is {}\n", b);
    assert_eq!(c, b); // assert_eq! is a macro, it will throw an error if the two values are not equal assert_eq!是一个宏，如果两个值不相等，它会抛出一个错误

    let (o, p, q, r, e);
    (o, p) = (1, 2);
    (q, .., r, _) = (1, 2, 3, 4, 5);
    Struct {e,..} = Struct {e : 5};
    assert_eq!((o,p,q,r,e), (1,2,1,4,5));

    const _MAX_POINTS: u32 = 100_000; // const is a constant, it can't be changed const是一个常量，它不能被改变 u32 is an unsigned 32-bit integer u32是一个无符号的32位整数

    let h = 5;
    print!("the value of h is {}\n", h);
    let h = h + 1; // shadowing, it can change the type of the variable 遮蔽，它可以改变变量的类型
    print!("the value of h is {}\n", h);
    {
        let h = h * 2; // shadowing, it can change the type of the variable in this area  遮蔽，它可以改变变量的类型，仅限当前作用域
        print!("the value of h is {}\n", h);
    }

    let spaces = "   "; // spaces is a string spaces是一个字符串 ;"   " is a string literal "   "是一个字符串字面量
    print!("the value of spaces is {}\n", spaces);
    let spaces = spaces.len(); // spaces.len() is a function, it can get the length of a string spaces.len()是一个函数，它可以获取字符串的长度,usize is an unsigned integer, it's size depends on the computer architecture usize是一个无符号的整数，它的大小取决于计算机的架构
    print!("the value of spaces is {}\n", spaces);
}