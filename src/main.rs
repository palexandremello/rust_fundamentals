static GLOBAL_VARIABLE:u8 = 1;

fn sombra () {
    let a = 123;
    {
        let b = 456;
        println!("address b {:p}", &b);
        println!("address a {:p}", &a);

    }
    println!("address a {:p}", &a);
}

fn escopo() {
    let variavel:i32 = 300;
    println!("variavel = {}, address = {:p}", variavel, &variavel);
    let variavel:i32 = 301;
    println!("variavel = {}, address = {:p}", variavel, &variavel);
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
fn main() { 
    sombra();  
    escopo();
}