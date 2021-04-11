use std::io;

fn main() {
    println!("Input convering mode\n1: Celsius to Fahrenheit\n2: Fahrenheit to Celsius");
    let mut mode = String::new();
    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");
    let mode: u32 = mode.trim().parse().expect("Please input number!");

    if mode != 1 && mode != 2 {
        std::process::exit(1)
    }

    println!("Input temperature");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let temp: f64 = temp.trim().parse().expect("Please input number!");

    if mode == 1 {
        println!("{}°C equals to {}°F", temp, celsius_to_fahrenheit(temp));
    } else if mode == 2 {
        println!("{}°F equals to {}°C", temp, fahrenheit_to_celsius(temp));
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8 + 32.0).into()
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    ((fahrenheit - 32.0) / 1.8).into()
}
