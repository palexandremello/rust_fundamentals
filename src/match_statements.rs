fn main () {

    let language = "Python";

    let propouse = match language {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

    println!("{} {}", language, propouse);
}