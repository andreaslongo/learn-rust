#[cfg(test)]
mod tests {

    #[test]
    fn constants() {
        // They are always immutable and the type must be annotated.
        // They can be declared in global scope, e.g. outside of a function.
        // We can use a limited set of operations to write the value (e.g. *).
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        assert_eq!(THREE_HOURS_IN_SECONDS, 10800)
    }

    #[test]
    fn variables_and_mutability() {
        // Variables are immutable by default.
        // Without 'mut', this would trow a compile time error if we try to change the value later.
        let mut x = 5;
        assert_eq!(x, 5);
        x = 6;
        assert_eq!(x, 6);
    }

    #[test]
    fn shadowing() {
        // Declaring a new variable with the same name as a previous one.
        // We can perform transformations on a value but have the variable be immutable, e.g. change
        // its type.
        // E.g. this is useful if we want to reuse a name for other values.
        let y = 5;
        let y = y + 1;
        assert_eq!(y, 6);
        {
            let y = y * 2;
            assert_eq!(y, 12);
        }
        assert_eq!(y, 6);
    }
}
