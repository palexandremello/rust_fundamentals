

fn ownership() {
    let one_string = String::from("Stringzinha");
    let new_string = rouba(one_string);

    println!("{}", new_string);

}

fn rouba(string: String) -> String {
    println!("{}", string);

    string
}

fn main() {
    ownership()
}