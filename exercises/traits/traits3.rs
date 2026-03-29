// traits3.rs
//
// This is a setup similar to previous exercises. We'll add some default
// implementation to the trait. You might want to have a look back at the
// traits1/traits2 exercises.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.


pub trait Licensed {
    fn licensing_info(&self) -> String{
        String::from("Some information")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

fn main() {
    let some_software = SomeSoftware { version_number: 1 };
    let other_software = OtherSoftware { version_number: "v2".to_string() };

    println!("{} {}", some_software.licensing_info(), other_software.licensing_info());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensed() {
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware { version_number: "v2".to_string() };
        assert_eq!(some_software.licensing_info(), "Some information");
        assert_eq!(other_software.licensing_info(), "Some information");
    }
}
