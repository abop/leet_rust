pub mod n6_zigzag_conversion;
pub mod n5_longest_palindrome_substring;
pub mod lifetime;
pub mod r#move;

fn main() {
    n5_longest_palindrome_substring::main();
    n6_zigzag_conversion::main();
    r#move::main();
    lifetime::main();
}
