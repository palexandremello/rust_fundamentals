static GLOBAL_VARIABLE:u8 = 1;

fn main() {    
    let variavel:i32 = 128;
    println!("variavel = {}, tamanho = {}", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("{}", decimal);

    let  is_true:bool = false;
    println!("{} bytes", std::mem::size_of_val(&is_true));

    let word:char = 'C';


    println!("char = {} bytes", std::mem::size_of_val(&word));

    const PI:f32 = 3.14;
    println!("PI = {}", PI);
    println!("Global variable = {}", GLOBAL_VARIABLE);
}