use core::f64;
use std::io;

fn main() {
    println!();
    println!();
    println!();

    println!("Enter temperature in Fehrenheit to convert to Celsius");

    let mut input_temperature = String::new();

    io::stdin()
        .read_line(&mut input_temperature)
        .expect("Failed to read line");

    let input_temperature: f64 = input_temperature
        .trim()
        .parse()
        .expect("Temperature entered is not valid!");

    let output_temperature = (input_temperature - 32.0) * 5.0 / 9.0;

    println!("Temperature: {output_temperature}")
}
