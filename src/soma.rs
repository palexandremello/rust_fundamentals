
fn soma(a:i32, b:i32) -> i32 {

    println!("{} + {} = {}", a, b, a+b);
    a + b
}

fn main() {
    let soma_value:i32 = soma(2, 3);
    println!("{}", soma_value);
}