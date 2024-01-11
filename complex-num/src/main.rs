use num::complex::Complex;

fn main(){
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
}
