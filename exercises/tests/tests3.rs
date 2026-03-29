// tests3.rs
//
// In this exercise, you're testing a function that returns a boolean value.
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.


pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(0));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));
    }
}
