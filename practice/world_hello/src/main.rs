fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let s1 = String::from("Alice");
    let s2 = "Bob";

    greet(&s1);
    greet(s2);
}
