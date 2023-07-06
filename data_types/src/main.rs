// Rust is statically typed.
// It therefore must know the types of all variables at compile time.

#[test]
fn type_inference() {
    // The compiler can infer the type of data based on the value and how we use it.
    assert_eq!(get_type_of(&42), "i32");
    assert_eq!(get_type_of(&42.0), "f64");
    assert_eq!(get_type_of(&"a string"), "&str");
}

#[test]
fn type_annotations() {
    // When many types are possible, we must add type annotations
    let a_string = "42";
    let parsed_integer_with_annotation: u32 = a_string.parse().expect("Not a number!");
    assert_eq!(get_type_of(&parsed_integer_with_annotation), "u32");
}

// Scalar types
// They represent a single value.

#[test]
fn integer_types() {
    // An integer is a numbers without a fractional component.
    // Each variant has an explicit size which is the space it takes up in memory.
    // An integer can be either signed (value can be negative) or unsigned (value can only be
    // positive).
    let _unsigned_8bits_long_integer_min: u8 = 0;
    let _unsigned_8bits_long_integer_max: u8 = 255;

    let _signed_8bits_long_integer_min: i8 = -128;
    let _signed_8bits_long_integer_max: i8 = 127;

    // The inferred default types are usually a good start when unsure about the size needed.
    let default_integer = 255;
    assert_eq!(get_type_of(&default_integer), "i32");
}

#[test]
fn integer_literals() {
    let decimal = 98222;
    assert_eq!(decimal, 98222);

    let hex = 0xff;
    assert_eq!(hex, 255);

    let octal = 0o77;
    assert_eq!(octal, 63);

    let binary = 0b1010;
    assert_eq!(binary, 10);

    let byte = b'A'; // u8 only (octet) as 1 byte == 8 bits (usually)
    assert_eq!(byte, 65);

    // Literals can also use _ as a visual separator to make them easier to read.
    assert_eq!(98_222, 98222);
    assert_eq!(0b1111_0000, 0b11110000);
}

#[test]
#[should_panic]
#[allow(arithmetic_overflow)]
fn integer_overflow() {
    // This occurs during runtime, if we try to change a variable to a value outside the possible
    // range.

    let x: u8 = 255;
    let _ = x + 1;

    // If compiled in debug mode, the program will panic at runtime.
    // If compiled in release mode, the value will be wrapped around (two's complement wrapping).

    // We can explicitly handle overflows with methods from the standard library for primitive
    // numeric types: wrapping_*, checked_*, overflowing_* and saturating_*.
}

#[test]
fn floating_point_types() {
    // https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types
    assert!(false, "TODO");
}

///////////////////////////////////////////////////////////////////////////////
// Helper functions
///////////////////////////////////////////////////////////////////////////////

#[allow(unused)]
fn get_type_of<T>(_: &T) -> &'static str {
    let type_name = std::any::type_name::<T>();
    return type_name;
}

fn main() {
    println!("Usage: Look at the code in `src/main.rs` and run `cargo test`.");
}
