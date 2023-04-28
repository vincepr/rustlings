// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.


#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    ($va1:expr, $va2:expr) => {
        println!("Look at this other macro: {} + {} = {}", $va1, $va2, ($va1+$va2) );
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!(10, 45);
}
