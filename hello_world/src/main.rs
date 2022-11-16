fn main() {
    greet_hello()
}

fn greet_hello() {
    let chinese = "你好，世界";
    let english = "Hello, world";
    let languages = [chinese, english];
    for language in languages.iter() {
        println!("{}", &language);
    }
}