fn main() {
    println!("Usage: Look at the code in `src/main.rs` and run `cargo test`.");
}

#[test]
fn if_expressions() {
    // An `if` expression allows you to branch your code depending on conditions.
    let mut number;
    let mut result;

    number = 3;
    // Conditions evaluate to `true` or `false` - type `bool`
    if number < 5 {
        // If the condition is `true`, run this block of code.
        result = "condition was true";
    } else {
        // Else, if the condition is `false`, run this block of code.
        // This is optional.
        // If there is no `else` and the condition is `false`, the if block is skipped.
        result = "condition was false";
    }
    assert_eq!(result, "condition was true");

    number = 5;
    if number != 0 {
        result = "number was something other than zero";
    }
    assert_eq!(result, "number was something other than zero");
}
