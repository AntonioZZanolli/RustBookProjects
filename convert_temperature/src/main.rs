use std::io;

fn convert_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    println!(
        "Choose the number: \n
            1 - Convert to Celsius. \n
            2 - Convert to Fahrenheit"
    );

    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read option");

    let option: u32 = option.trim().parse().expect("Enter 1 or 2");

    println!("Select the temperature value:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read temperature");

    let value: f64 = input.trim().parse().expect("Please enter a number");

    match option {
        1 => {
            let result = convert_to_celsius(value);
            println!("{value} °F = {result} °C");
        }
        2 => {
            let result = convert_to_fahrenheit(value);
            println!("{value} °C = {result} °F ")
        }
        _ => {
            println!("Invalid option. Choose 1 or 2.")
        }
    }

}
