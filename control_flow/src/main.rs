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
    let mut counter = 0;

    loop {
        // On every iteration of the loop, we add 1 to the `counter` variable.
        counter += 1;

        // If we are at 10 iterations, we want the program to stop executing the loop.
        // This is our stopping condition.
        // If we do not break, the loop block runs forever until we somehow stop the program.
        if counter == 10 {
            break;
        }
    }

    assert_eq!(counter, 10);

    // We can also tell the program to skip over code in a loop that we don't want to execute with
    // the `continue` keyword.
    // When the program reaches a `continue`, it will immediately stop the current iteration of the
    // loop and start with the next iteration.

    // One use case of a loop is to retry an operation you know might fail, such as checking
    // whether a thread has completed its job.
}

#[test]
fn returning_values_from_loops() {
    // You can add values you want to pass out of the loop after the `break` expression.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // Note the semicolon here to end the statement that assigns the value to result.

    assert_eq!(result, 20);
}

#[test]
fn loop_labels_to_disambiguate_between_multiple_loops() {
    // If we have nested loops, `break` and `continue` apply to the innermost loop at that point.
    // We can label our loops and then use that label with `break` or `continue` to apply them on a
    // different loop.
    let mut count = 0;

    'counting_up: loop {
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    assert_eq!(count, 2);
}

#[test]
fn conditional_loops_with_while() {
    // We often want a loop to run as long as some condition is met.
    let mut number = 3;

    loop {
        if number == 0 {
            break;
        } else {
            number -= 1;
        }
    }

    assert_eq!(number, 0);

    // This pattern is so common, that Rust has a built-in language construct for it, called a
    // `while` loop.
    // We can rewrite the loop above as a `while` loop which eliminates a lot of nesting.
    let mut number = 3;

    while number != 0 {
        number -= 1;
    }

    assert_eq!(number, 0);
}

#[test]
fn looping_through_a_collection_with_for() {
    // We can use a `while` loop to iterate over the elements of a collection, such as an array.
    // We locate elements in the array by its index position.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    let mut vec = Vec::new();

    // The array has 5 elements
    // The elements are at index position 0..4 from first to last element.
    // We end the loop as soon as we reach index 5, because 5 would be out of range for the array.
    while index < 5 {
        // Get the value of the element at the given index from array `a`.
        let value = a[index];
        // Append the value to vector `vec`
        vec.push(value);
        // Go to the next element in the following iteration.
        index += 1;
    }
    assert_eq!(vec, [10, 20, 30, 40, 50]);

    // But the approach above is error prone.
    // We could cause the program to panic if the index value or test condition is incorrect
    // We could miss elements if we don't go far enough in the array.
    // It is also inefficient, because we have to check the index condition on every iteration.

    // Rust has a better way for executing code for each item in a collection. You can use a `for`
    // loop for this and improve the code above.
    let a = [10, 20, 30, 40, 50];
    let mut vec = Vec::new();

    for element in a {
        vec.push(element);
    }

    assert_eq!(vec, [10, 20, 30, 40, 50]);

    // `for` loops are the most commonly used loop constructs in Rust because they are generally
    // safe and concise.
    // You can use it in other situations, like e.g. instead of a `while` loop as in the countdown
    // from our while loop example above.
    let mut number = 3;
    for _ in (1..4).rev() {
        number -= 1;
    }
    assert_eq!(number, 0);

    // This uses a `Range` provided by the standard library.
    // It generates numbers in sequence starting from one number and ending before another number.
    // `rev` reverses the range.
    let mut vec = Vec::new();
    for element in (1..4).rev() {
        vec.push(element);
    }
    assert_eq!(vec, [3, 2, 1]);
}
