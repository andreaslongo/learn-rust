// Converts temperatures between Fahrenheit and Celsius.
fn main() {
    let f = 68.0;
    let c = to_celsius(f);
    println!("{f} degrees Fahrenheit is equal to {c} degrees Celsius.");

    let c = 20.0;
    let f = to_fahrenheit(c);
    println!("{c} degrees Celsius is equal to {f} degrees Fahrenheit.");
}

#[test]
fn convert_fahrenheit_to_celsius() {
    let fahrenheit = 60.0;
    let celsius = to_celsius(fahrenheit);
    assert_eq!(celsius, 15.555555);

    let fahrenheit = 70.0;
    let celsius = to_celsius(fahrenheit);
    assert_eq!(celsius, 21.1111111);
}

fn to_celsius(fahrenheit: f32) -> f32 {
    // Single precision should be enough for a human readable temperature.
    (fahrenheit - 32.0) * 5.0 / 9.0
}

#[test]
fn convert_celsius_to_fahrenheit() {
    let celsius = 20.0;
    let fahrenheit = to_fahrenheit(celsius);
    assert_eq!(fahrenheit, 68.0);

    let celsius = 15.0;
    let fahrenheit = to_fahrenheit(celsius);
    assert_eq!(fahrenheit, 59.0);
}

fn to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}
