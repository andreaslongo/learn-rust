// Converts temperatures between Fahrenheit and Celsius.
fn main() {
    let f = 60.0;
    let c = to_celsius(f);
    println!("{f} degrees Fahrenheit is equal to {c} degrees Celsius.");
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
    // Single precision should be enough for displaying a human readable temperature.
    (fahrenheit - 32.0) * 5.0 / 9.0
}
