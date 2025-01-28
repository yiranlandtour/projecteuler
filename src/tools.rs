
//判断回文
pub fn is_palindrome(n: u64) -> bool {
    // Convert the number to a string for easier manipulation
    let num_str = n.to_string();
    // Compare the string with its reverse
    num_str == num_str.chars().rev().collect::<String>()
}