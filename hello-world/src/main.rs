fn greet(){
    let chinese = "你好，世界";
    let english = "hello,world";
    let regions = [chinese, english];
    for region in regions.iter(){
        println!("{}", &region);
        let a = 1;
        print!("{}", a);
    }
}
fn main(){
    greet();
}