struct Dog {
    name: String,
    age: u32,
    owner: String,
}

// Implementing an in-built trait ToString on the Dog struct
impl ToString for Dog {
    fn to_string(&self) -> String {
        return format!(
            "{} is a {} year old dog who belongs to {}.",
            self.name, self.age, self.owner
        );
    }
}

fn main() {
    let dog = Dog {
        name: "Frodo".to_string(),
        age: 3,
        owner: "Maryam".to_string(),
    };
    println!("{}", dog.to_string());
}
