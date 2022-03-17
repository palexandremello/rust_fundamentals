

fn errors() {

    match result() {
        Ok(s) => println!("{}", s),
        Err(number) => println!("Error {}", number)
    }

}

fn result() -> Result<String, u8> {
    Err(42)
}
fn main() {
    errors();
}