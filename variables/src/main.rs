// Constants
// They are always immutable and the type must be annotated.
// They can be declared in global scope (e.g. outside of main() in this example).
// We can use a limited set of operations to write the value (e.g. *).
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("The value of the constant is: {THREE_HOURS_IN_SECONDS}");

    // Variables and Mutability
    // Variables are immutable by default.
    // Without 'mut', this would trow a compile time error if we try to change the value later.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    // Declaring a new variable with the same name as a previous one.
    // We can perform transformations on a value but have the variable be immutable, e.g. change
    // its type.
    // E.g. this is useful if we want to reuse a name for other values.
    let y = 5;
    let y = y + 1;
    println!("The value of y is: {y}");
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");
}
