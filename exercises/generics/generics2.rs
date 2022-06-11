// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` for hints!

struct Wrapper<T> {
    value: T,
    other: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T, other: T) -> Self {
        Wrapper { value: value, other: other }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42, 36).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo", "Bar").value, "Foo");
    }
}
