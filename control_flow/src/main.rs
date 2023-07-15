fn main() {
    println!("Usage: Look at the code in `src/main.rs` and run `cargo test`.");
}

// `if` Expressions
// An `if` expression allows you to branch your code depending on conditions.

#[test]
fn conditions() {
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

#[test]
fn handling_multiple_conditions_with_else_if() {
    let number = 6;
    let result;

    // Rust executes the block for the first `true` condition.
    // Once it finds one, it doesn't even check the remaining conditions.
    if number % 4 == 0 {
        result = "number is divisible by 4";
    } else if number % 3 == 0 {
        result = "number is divisible by 3";
    } else if number % 2 == 0 {
        result = "number is divisible by 2";
    } else {
        result = "number is not divisible by 4,4, or 2";
    }

    assert_eq!(result, "number is divisible by 3");

    // Using too many `else if` expressions can clutter your code.
    // If you have more than one, you might want to refactor your code.
    // E.g. use a `match` construct instead.
}

#[test]
fn using_if_in_a_let_statement() {
    // `if` is an expression.
    // We can use it on the right side of a `let` statement to conditionally assign values.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    assert_eq!(number, 5);

    // This works because blocks of code evaluate to the last expression in them, and numbers by
    // themselves are also expressions.
    // Note that the result values for each arm of the `if` must be the same type.
}

// Repetition with Loops
// Loops allow you to execute a block of code more than once.

#[test]
fn repeating_code_with_the_loop_keyword() {
    // The `loop` keyword lets you execute a block of code forever until you tell it to stop.
    let mut iteration_count = 0;

    loop {
        // Every time this block is repeated, we count +1.
        iteration_count = iteration_count + 1;

        // If we are at 10 iterations, we want the program to stop executing the loop.
        // This is our stopping condition.
        // If we do not break, the loop block runs forever until we somehow stop the program.
        if iteration_count == 10 {
            break;
        }
    }

    assert_eq!(iteration_count, 10);

    // We can also tell the program to skip over code in a loop that we don't want to execute with
    // the `continue` keyword.
    // When the program reaches a `continue`, it will immediately stop the current iteration of the
    // loop and start with the next iteration.
}
