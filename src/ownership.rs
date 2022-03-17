

fn ownership() {
    let mut one_string = String::from("Stringzinha");
    rouba(&mut one_string);

    println!("{}", one_string);

}

fn rouba(string: &mut String) {
    println!("{}", string);
}

fn main() {
    ownership();
}