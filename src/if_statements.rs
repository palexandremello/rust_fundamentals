

fn main() {
    let age: u8 = 14;
    let is_root:bool = true;

    if age > 18 || age >= 14 && is_root {
        println!("Ok");
    } else {
        println!("not okay");
    }

    let condition = if is_root { "is root"} else {"is not root"};

    println!("{}", condition);

}