// traits5.rs
//
// This is a setup similar to previous exercises. We'll add some default
// implementation to the trait. You might want to have a look back at the
// traits1/traits2 exercises.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.


pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
struct OtherStruct;

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_func_true() {
        assert!(some_func(SomeStruct));
    }

    #[test]
    fn some_func_false() {
        assert!(some_func(OtherStruct));
    }
}
