// traits4.rs
//
// This is a setup similar to previous exercises. We'll add some default
// implementation to the trait. You might want to have a look back at the
// traits1/traits2 exercises.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.


pub trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Some information")
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_same() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }
}
