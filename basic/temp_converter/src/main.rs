
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    let mut temperature = String::new();
    let mut temperature_type = String::new();

    println!("Enter the temperature to convert: ");
    std::io::stdin().read_line(&mut temperature).unwrap();
    let temperature: f64 = temperature.trim().parse().unwrap();

    println!("Enter the temperature type (C for Celsius, F for Fahrenheit): ");
    std::io::stdin().read_line(&mut temperature_type).unwrap();
    let temperature_type = temperature_type.trim();

    if temperature_type == "C" {
        let fahrenheit = celsius_to_fahrenheit(temperature);
        println!("{} degrees Celsius is equal to {} degrees Fahrenheit", temperature, fahrenheit);
    } else if temperature_type == "F" {
        let celsius = fahrenheit_to_celsius(temperature);
        println!("{} degrees Fahrenheit is equal to {} degrees Celsius", temperature, celsius);
    } else {
        println!("Invalid temperature type");
    }
}
