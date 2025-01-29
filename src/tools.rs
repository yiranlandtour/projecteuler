use num_bigint::BigInt;
use num_traits::{Zero, One, Signed, ToPrimitive};


pub fn sum_digits_bignum(n: &BigInt) -> u64 {
    let mut sum = 0u64;
    let ten = BigInt::from(10u32);
    let mut n = n.abs(); 


    if n.is_zero() {
        return 0;
    }


    while n > BigInt::zero() {
        let remainder = &n % &ten; 
        sum += remainder.to_u64().unwrap(); 
        n /= &ten; 
    }
    sum
}

pub fn sum_digits(mut n: u64) -> u64 {
    let mut sum =0u64;

    while n > 0 {
        let remainder = n % 10; 
        sum += remainder; 
        n /= 10; 
    }
    sum
}


// pub fn is_palindrome(n: u64) -> bool {
//     // Convert the number to a string for easier manipulation
//     let num_str = n.to_string();
//     // Compare the string with its reverse
//     num_str == num_str.chars().rev().collect::<String>()
// }