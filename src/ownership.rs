

fn ownership() {
    let one_string = String::from("Stringzinha");
    rouba(&one_string);

    println!("{}", one_string);

}

fn rouba(string: &String) {
    println!("{}", string);
}

fn main() {
    ownership();
}