#[cfg(test)]
mod tests {
    // The concept of ownership allows a Rust program to automatically manage memory without the
    // runtime overhead of a garbage collector.

    #[test]
    fn the_stack_and_the_heap() {
        // The stack is a last in, first out (LIFO) data structure for storing things in memory.
        // All data stored on the stack must have a known, fixed size.
        // Data with an unknown size at compile time or a size that might change must be stored on
        // the heap instead.

        // Assignment either pushes data onto the stack or allocates memory on the heap depending
        // on the value to be assigned.

        // This assigns a value of a known, fixed size to a variable.
        // The value is pushed onto the stack.
        let data_on_stack: i32 = 100;

        // This assigns a value of an unknown, dynamic size to a variable.
        // Memory is allocated on the heap to store the value.
        let data_on_heap: String = String::from("This string value can change during runtime");
    }
}
