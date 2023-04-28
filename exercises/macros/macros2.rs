// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// YOU HAVE TO DEFINE IT BEFORE/ABOVE IT (sequetially, like JS closures etc...)
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
