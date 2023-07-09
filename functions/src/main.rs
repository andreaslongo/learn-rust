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

#[test]
fn statements_and_expressions() {
    // https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions
    assert!(false, "TODO");
    // Rust is an expression-based language,
    // Function bodies are made up of a series of statements optionally ending in an expression.
    // Statements are instructions that perform some action and do not return a value.
    // Creating a variable and assigning a value to it with the let keyword is a statement.
    // Function definitions are also statements
    fn function() {
        let y = 6;
    }
    // Expressions evaluate to a resultant value. Let’s look at some examples.
    //
}

///////////////////////////////////////////////////////////////////////////////
// Helper functions
///////////////////////////////////////////////////////////////////////////////

fn main() {
    println!("Usage: Look at the code in `src/main.rs` and run `cargo test`.");
}
