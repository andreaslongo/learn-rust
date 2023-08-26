#[cfg(test)]
mod tests {
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
    #[allow(unused_must_use)]
    fn statements_and_expressions() {
        // Rust is an expression-based language.
        // Function bodies are made up of a series of statements optionally ending in an expression.

        // Statements are instructions that perform some action and do not return a value.
        // Creating a variable and assigning a value to it with the `let` keyword is a statement.
        // Function definitions are also statements.
        fn function() {
            let _x = 6;
        }

        // Constructs like the following do not work in Rust, because assignment is a statement and
        // statements don't return values - so here x has nothing to bind to.
        // let x = (let y = 6);
        // This works in other languages e.g. C and Python:
        // x = y = 6

        // Expressions evaluate to a resultant value.
        // This expression evaluates to the value 11:
        5 + 6;
        assert_eq!(5 + 6, 11);

        // Expressions can be part of statements.
        let x = 5 + 6;
        let y = 6; // The 6 in this statement is an expression that evaluates to the value 6.
        assert_eq!(x, 11);
        assert_eq!(y, 6);

        // Calling a function is an expression.
        function();

        // Calling a macro is an expression.
        println!();

        // A new scope block created with curly brackets is an expression.
        let z = {
            let x = 3;
            x + 1 // Note the missing semicolon here.
        };
        assert_eq!(z, 4);

        // Expressions do not include ending semicolons.
        // If you add a semicolon to the end of an expression, you turn it into a statement.
        // It will then not return a value.
    }

    #[test]
    fn functions_with_return_values() {
        // Functions can return values to the code that calls them.
        // We must declare the type of a return value after an arrow.
        fn five() -> i32 {
            5
        }
        let returned_value = five();
        assert_eq!(returned_value, 5);

        // Functions return the value of the last expression in their body implicitly.
        fn ten() -> i32 {
            5 + 5
        }
        let y = ten();
        assert_eq!(y, 10);

        // We can return early from a function with the return keyword.
        #[allow(unreachable_code)]
        fn early_return_ten() -> i32 {
            let ten = 10;
            return ten; // any code following this is unreachable
            13
        }
        let z = early_return_ten();
        assert_eq!(z, 10);

        // This function will return the value of x + 1
        fn plus_one(x: i32) -> i32 {
            x + 1
        }
        let a = plus_one(5);
        assert_eq!(a, 6);

        // But if we change the expression to a statement, we get a compile-time error.
        // The definition of the function says it will return an `i32`, but statements don't evaluate
        // to a value, which is expressed by `()`, the unit type.
        // So nothing is returned which contradicts the function definition and results in an error.
        // fn plus_one(x: i32) -> i32 {
        //     x + 1;  // Note the added semicolon here
        // }
    }
}
