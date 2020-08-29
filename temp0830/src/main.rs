struct Person {
    name: String,
    age: u32,
}

fn main() {
    println!("Hello, world!");
    let p = Person {
        name: String::from("John"),
        age: 8,
    };
}
