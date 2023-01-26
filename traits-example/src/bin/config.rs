// struct for project configuration
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub author: String,
}

// trait for project configuration
pub trait Config {
    fn new() -> Self;
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: String);
}

// Implementing the Config trait on the ProjectConfig struct
impl Config for ProjectConfig {
    fn new() -> Self {
        ProjectConfig {
            name: "My Project".to_string(),
            version: "0.1.0".to_string(),
            author: "John Doe".to_string(),
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        match key {
            "name" => Some(self.name.clone()),
            "version" => Some(self.version.clone()),
            "author" => Some(self.author.clone()),
            _ => None,
        }
    }

    fn set(&mut self, key: &str, value: String) {
        match key {
            "name" => self.name = value,
            "version" => self.version = value,
            "author" => self.author = value,
            _ => (),
        }
    }
}

fn main() {
    // create a new project configuration
    let mut config = ProjectConfig::new();

    // get the project name
    println!("Project name: {}", config.get("name").unwrap());

    // set the project name
    config.set("name", "My New Project".to_string());

    // get the project name
    println!("Project name: {}", config.get("name").unwrap());
}
