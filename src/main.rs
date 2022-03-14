fn main() {
    let variavel:i32 = 128;
    println!("variavel = {}, tamanho = {}", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("{}", decimal);
}