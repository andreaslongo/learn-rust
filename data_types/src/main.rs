fn main() {
    println!("Usage: Look at the code in `src/main.rs` and run `cargo test`.");
}

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

    let byte = b'A'; // `u8` only (octet) as 1 byte == 8 bits (usually)
    assert_eq!(byte, 65);

    // Literals can also use `_` as a visual separator to make them easier to read.
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
    // numeric types: `wrapping_*`, `checked_*`, `overflowing_*` and `saturating_*`.
}

#[test]
fn floating_point_types() {
    // A floating-point number is a number with a decimal point.
    // There are two float sizes and the size dictates the precision in which numbers can be
    // represented.
    let _single_precision_float: f32 = 3.0;
    let _double_precision_float: f64 = 4.3;

    // All floating-point types are signed.
    let _always_signed_float: f32 = -3.0;

    // `f64` is the default inferred type because on modern CPUs, it's roughly the same speed as
    // `f32` but is capable of more precision.
    let default_float = 2.0;
    assert_eq!(get_type_of(&default_float), "f64");
}

#[test]
fn numeric_operations() {
    // Each expression in these statements uses a mathematical operator and evaluates to a single
    // value.

    // addition
    let sum = 5 + 10;
    assert_eq!(sum, 15);

    // subtraction
    let difference = 95.5 - 4.3;
    assert_eq!(difference, 91.2);

    // multiplication
    let product = 4 * 30;
    assert_eq!(product, 120);

    // division
    let quotient = 56.7 / 32.2;
    assert_eq!(quotient, 1.7608695652173911);

    // Integer division truncates toward zero to the nearest integer.
    let truncated = -5 / 3;
    assert_eq!(truncated, -1);

    // remainder
    let remainder = 43 % 5;
    assert_eq!(remainder, 3);
}

#[test]
fn the_boolean_type() {
    // A boolean type has two possible values: `true` and `false`.
    // They are one byte in size in memory.
    // Boolean values are mainly used through conditionals, e.g. `if` expressions.
    let t = true;
    assert_eq!(t, true);

    let f: bool = false; // with explicit type annotation
    assert_eq!(f, false);
}

#[test]
fn the_character_type() {
    // A primitive alphabetic type.
    // It is four bytes in size and represents a Unicode Scalar Value.
    // It can therefore represent much more than ASCII.
    // `char` literals use single quotes.
    // `string` literals use double quotes.
    let c = 'z';
    assert_eq!(c, 'z');

    let z: char = 'ℤ'; // with explicit type annotation
    assert_eq!(z, 'ℤ');

    let heart_eyed_cat = '😻';
    assert_eq!(heart_eyed_cat, '😻');
}

// Compound types
// They can group multiple values into one type.

#[test]
fn the_tuple_type() {
    // A group of values with various types.
    // The values in the tuple don’t have to be the same type.
    // Tuples have a fixed length, they cannot grow or shrink once declared.
    let tuple = (500, 6.4, 1);

    // A tuple with optional type annotations.
    let _annotated_tuple: (i32, f64, u8) = (500, 6.4, 1);

    // We can use destructuring to get the individual values.
    // This uses pattern matching to break a single tuple into separate parts.
    let (x, y, z) = tuple;
    assert_eq!(x, 500);
    assert_eq!(y, 6.4);
    assert_eq!(z, 1);

    // We can access a tuple element directly by using its index.
    // The first index is 0.
    assert_eq!(tuple.0, 500);
    assert_eq!(tuple.1, 6.4);
    assert_eq!(tuple.2, 1);

    // The tuple without any values has a special name: unit.
    // This value and its corresponding type are both written `()`.
    // It represents an empty value or an empty return type.
    // Expressions implicitly return the unit value if they don’t return any other value.
    let unit: () = ();
    assert_eq!(get_type_of(&unit), "()");
}

#[test]
#[should_panic]
#[allow(unconditional_panic)]
fn the_array_type() {
    // A group of values with the same type.
    // Every value in the array must have the same type.
    // Arrays have a fixed length, they cannot grow or shrink once declared.
    // An array is a single chunk of memory of a known, fixed size that can be allocated on the
    // stack.
    let array = [1, 2, 3, 4, 5];

    // A vector is a similar collection type provided by the standard library
    // It can grow or shrink in size and is therefore more flexible than an array.
    // Better use a vector if unsure whether to use an array or a vector.

    // Arrays are useful when we want our data allocated on the stack rather than the heap or when
    // we want to ensure we always have a fixed number of elements or when we know the number of
    // elements will not need to change.
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // An array with optional type annotations.
    // The annotation contains the type of each element followed by the number of elements in the
    // array.
    let _annotated_array: [i32; 5] = [1, 2, 3, 4, 5];

    // We can initialize an array with the same value for each element.
    let initialized_array = [3; 5];
    assert_eq!(initialized_array, [3, 3, 3, 3, 3]);

    // We can access elements of an array using indexing
    assert_eq!(array[0], 1);
    assert_eq!(array[1], 2);
    assert_eq!(array[4], 5);

    // If we try to access an element past the end of an array, the program will panic at runtime
    // to prevent invalid memory access.
    assert_eq!(array[5], 999);
}

///////////////////////////////////////////////////////////////////////////////
// Helper functions
///////////////////////////////////////////////////////////////////////////////

#[allow(unused)]
fn get_type_of<T>(_: &T) -> &'static str {
    let type_name = std::any::type_name::<T>();
    type_name
}
