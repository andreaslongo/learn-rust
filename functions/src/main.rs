#[test]
fn functions() {
    // Declaring a new function
    fn another_function() -> &'static str {
        "a string value"
    }

    // Calling a function
    another_function();

    // Calling a function and binding the returned value to a new variable
    let result = another_function();
    assert_eq!(result, "a string value");
}

#[test]
fn parameters() {
    // Parameters are special variables that are part of a function's signature.
    fn function_with_parameter(x: i32) -> String {
        format!("You passed {x} as an argument to my parameter.")
    }

    // When calling a function, we can provide concrete values for it's parameters to input data.
    // The values passed in when you call a function are called arguments.
    let result = function_with_parameter(5);
    assert_eq!(result, "You passed 5 as an argument to my parameter.");

    // Defining multiple parameters
    fn function_with_two_parameters(value: i32, unit_label: char) -> String {
        format!("The measurement is: {value}{unit_label}")
    }

    let result = function_with_two_parameters(5, 'h');
    assert_eq!(result, "The measurement is: 5h");
}

///////////////////////////////////////////////////////////////////////////////
// Helper functions
///////////////////////////////////////////////////////////////////////////////

fn main() {
    println!("Usage: Look at the code in `src/main.rs` and run `cargo test`.");
}
