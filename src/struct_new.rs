struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Self {
        Person {
            name: name.to_string(),
            age: age
        }
    }
}

pub fn main() {
    let taro = Person::new("Taro", 20);
    println!("{} is {} years old.", taro.name, taro.age);
}