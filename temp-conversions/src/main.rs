use std::io::stdin;


fn main() {
    // Decide whether celsius -> fahrenheit or the converse
    let mut celsius_bool: String = String::new();
    println!("Enter true if your initial tempreture reading is in celsius, false if fahrenheight: ");
    stdin().read_line(&mut celsius_bool).expect("failed to read line");
    // Before parsing convert to lowercase, bools in rust are lowercase by default
    let celsius_bool: bool = celsius_bool.trim().to_lowercase().parse().expect("Expected bool type");

    // Read in temperature 
    let mut temperature: String = String::new();
    println!("Enter your temperature reading: ");
    stdin().read_line(&mut temperature).expect("failed to read line");
    let temperature: f64 = temperature.trim().parse().expect("Expected float64 type");

    // if-else logic for conversions, simple stuff
    if celsius_bool {
        let f_temperature: f64 = celsius_to_fahrenheit(temperature);
        println!("Temperature in fahrenheight is: {f_temperature}");
    }
    else {
        let c_temperature: f64 = fahrenheit_to_celsius(temperature);
        println!("Temperature in celsius is: {c_temperature}");
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (9.0/5.0)*c + 32.0 // celsius -> fahrenheit formula, googlable 
    // Note we need .0 to preserve types when doing arithmetic opps, or we try and add f64 with u8 or something, no good!
}

fn fahrenheit_to_celsius(f: f64) -> f64{
    (5.0/9.0)*(f - 32.0) // Inverse of prior formula, same comment on the .0's 
}
