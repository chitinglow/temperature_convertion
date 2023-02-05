use std::io;
use std::string::String;

fn main() {
    loop {
        println!("Please select a type of conversion:");
        println!("1 for converting Fahrenheit to Celsius");
        println!("2 for converting Celsius to Fahrenheit");

        let mut conversion_type = String::new();
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line");
        let conversion_type = conversion_type.trim();
        let conversion_type = match conversion_type {
            "1" => 1,
            "2" => 2,
            _ => {
                println!("Please input 1 or 2!");
                continue;
            }
        };

        println!("Please type the tempereture.");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Can't parse input. Please input float type value.");
                return;
            }
        };

        if conversion_type == 1 {
            let celcius = farenheit_to_celcius(temperature);
            println!("Farenheit {temperature} is: {celcius} Celcius");
            break;
        } else {
            let farenheit = celcius_to_farenheit(temperature);
            println!("Celcius {temperature} is: {farenheit} Farenheit");
            break;
        }
    }
}

fn farenheit_to_celcius(x: f64) -> f64 {
    (x - 32.) * 0.5556
}

fn celcius_to_farenheit(x: f64) -> f64 {
    x * 1.8 + 32.
}
