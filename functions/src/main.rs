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

///////////////////////////////////////////////////////////////////////////////
// Helper functions
///////////////////////////////////////////////////////////////////////////////

fn main() {
    println!("Usage: Look at the code in `src/main.rs` and run `cargo test`.");
}
