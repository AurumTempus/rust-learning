fn main() {
    let temperature: i16 = -10;
    println!("Temperature: {}C", temperature);

    let temperature: i16 = temperature + 20;
    println!("Warmed temperature: {}C", temperature);

    let mut pi_approx: f64 = 3.14;
    pi_approx = pi_approx + 0.00159;
    println!("Approximate Pi {}", pi_approx);

    let is_positive: bool = true;
    let unit = '°';

    println!("Is warmed temp positive? {}, Unit {}", is_positive, unit);

    let user = String::from("Horry");
    println!("Calculated by: {}", user);
}
