fn main() {
    let multiplicador:u8 = 5;
    let mut contador:u8 = 0;

    while contador < 10 {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    loop {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
        
        if contador == 10 {
            break;
        }
    }

}