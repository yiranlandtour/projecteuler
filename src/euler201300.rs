fn match_pattern(n: u64) -> bool {
    let mut n = n;
    let mut digit = 9;

    while n > 0 {
        if n % 10 != digit {
            return false;
        }
        n /= 100;
        digit -= 1;
    }
    true
}

pub fn euler206()->u64{
    let upper_bound: u64 = (1929394959697989990u64 as f64).sqrt() as u64;
    let lower_bound: u64 = (1020304050607080900u64 as f64).sqrt() as u64;
    
    let mut x = if upper_bound % 2 == 0 {
        upper_bound
    } else {
        upper_bound - 1
    };

    while x >= lower_bound {
        // Check both x and x-2 because they both end in the same digit when squared.
        if match_pattern(x * x) {
            println!("Found x: {}", x);
            return x;
            break;
        }
        x -= 10;
    }
    return 0;
}

