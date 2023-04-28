// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.


fn returnsString() -> String{
    String::from("Rust")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        let equal = String::from("Rust");
        let also_equal = "Rust";
        assert_eq!(equal, returnsString());
        assert_eq!(also_equal, returnsString());
    }
}
