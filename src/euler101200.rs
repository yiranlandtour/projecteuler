use std::{fs, string};

extern crate num_bigint;
extern crate num_traits;



use num_bigint::{BigInt,BigUint};
use num_traits::{One, Zero};
use std::collections::{HashSet,HashMap};
    // let a = crate::euler::sum_of_divisors(123);
    // println!("sum of divisors of 123 is {}", a);

use crate::tools::{sum_digits,sum_digits_bignum}; 

fn triangle_area(a:f32,b:f32,c:f32)->f32{
    let s = (a+b+c)/2.0;
    let area = (s*(s-a)*(s-b)*(s-c)).sqrt();
    area
}

fn triangle_area_point(a:f32,b:f32,c:f32,d:f32,e:f32,f:f32)->f32{
    (a*(d-f) + c*(f-b) + e*(b-d)).abs()/2.0
}
pub fn euler102()->u32{
    let mut res = 0;
    let s = fs::read_to_string("source/102.txt").expect("Unable to read file");
    let matrix: Vec<Vec<f32>> = s.lines()
        .map(|line| line.split(',')
            .filter_map(|x| x.parse().ok())
            .collect::<Vec<f32>>())
        .collect();
    println!("matrix is {:?}", matrix);

    for i in 0..matrix.len(){
        let x1 = matrix[i][0];
        let y1 = matrix[i][1];
        let x2 = matrix[i][2];
        let y2 = matrix[i][3];
        let x3 = matrix[i][4];
        let y3 = matrix[i][5];

        // let a = ((x2-x1)*(x2-x1) + (y2-y1)*(y2-y1)).sqrt();
        // let b = ((x3-x2)*(x3-x2) + (y3-y2)*(y3-y2)).sqrt();
        // let c = ((x1-x3)*(x1-x3) + (y1-y3)*(y1-y3)).sqrt();
        // let area = triangle_area(a, b, c);

        // if triangle_area(a, (x1*x1+y1*y1).sqrt(), (x2*x2+y2*y2).sqrt()) + triangle_area(b, (x3*x3+y3*y3).sqrt(), (x2*x2+y2*y2).sqrt()) + triangle_area(c, (x1*x1+y1*y1).sqrt(), (x3*x3+y3*y3).sqrt()) == area{
        //     println!("{:?},{:?}",x1,y1);
        //     res += 1;
        // }
        if triangle_area_point(x1, y1, x2, y2, x3, y3) == triangle_area_point(0.0, 0.0, x2, y2, x3, y3) + triangle_area_point(x1, y1, 0.0, 0.0, x3, y3) + triangle_area_point(x1, y1, x2, y2, 0.0, 0.0){
            // println!("{:?},{:?}",x1,y1);
            res += 1;
        }
    }
    res
}

fn is_pandigital(n: u64) -> bool {
    let mut digits = HashSet::new();
    let mut num = n;
    while num > 0 {
        digits.insert(num % 10);
        num /= 10;
    }
    digits.len() == 9 && !digits.contains(&0)
}

pub fn euler104() {
    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();
    let mut index = 1;

    let base: BigUint = "1000000000".parse().unwrap(); // For last 9 digits

    loop {
        index += 1;
        let c = &a + &b;

        a = b.clone();
        b = c.clone();

        let last_nine_digits: u64 = (c.clone() % base.clone()).to_str_radix(10).parse().unwrap();

        let c_str = c.to_str_radix(10);
        let first_nine_digits: u64 = c_str.chars().take(9).collect::<String>().parse().unwrap();

        if is_pandigital(last_nine_digits) && is_pandigital(first_nine_digits) {
            println!("Found k: {}", index);
            break;
        }
    }
}

fn condition_met(last_nine: u64, n: u64, log_phi: f64, log_root_5: f64) -> bool {
    // if !is_pandigital(last_nine) {
    //     return false;
    // }
    // println!("{:?}",n);
    let log_fib_n = n as f64 * log_phi - log_root_5;
    let log_fib_n_frac_part = log_fib_n - log_fib_n.floor();
    let first_nine = (log_fib_n_frac_part.exp2() * 1e8).floor() as u64;
    
    if n < 100{
        println!("{:?}",first_nine);
    }
    if is_pandigital(first_nine){
        println!("{:?}",n);
    }
    // return is_pandigital(first_nine);
    false
}

pub fn euler104c(){
    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();
    let base: BigUint = 10u64.pow(10).into();
    
    let log_phi = 0.5 * (1.0 + 5f64.sqrt()).log2();
    let log_root_5 = 0.5 * 5f64.log2();
    
    let mut n = 2u64;

    loop {
        let c = &a + &b;
        let last_nine = &c % &base;
        
        if condition_met(last_nine.to_string().parse::<u64>().unwrap(), n, log_phi, log_root_5) {
            println!("Found it: {}", n);
            break;
        }

        a = b.clone();
        b = c;
        n += 1;
    }
}


pub fn euler112() -> u64{

    let target_proportion = 0.99;
    let mut bouncy_count = 0;
    let mut total_count = 0;

    for n in 1.. {
        total_count += 1;
        if is_bouncy(n) {
            bouncy_count += 1;
        }

        let proportion = bouncy_count as f64 / total_count as f64;
        if proportion >= target_proportion {
            println!("The least number for which the proportion of bouncy numbers is exactly 99% is: {}", n);
            return n;
        }
    }
    return 0;
}

fn is_bouncy(n: u64) -> bool {
    let digits: Vec<u8> = n.to_string().bytes().map(|b| b - b'0').collect();
    //println!("{:?},{:?}",digits,digits.windows(2));
    let increasing = digits.windows(2).all(|w| w[0] <= w[1]);
    let decreasing = digits.windows(2).all(|w| w[0] >= w[1]);

    !increasing && !decreasing
}

// pub fn euler125() -> u64{
    
// }

pub fn euler109_b() -> i32 {
    let mut count = 10;
    let mut start = BigInt::from(614656);
    while count < 30 {
        start += BigInt::one();
        let he = sum_digits_bignum(&start); 
        
        let mut temp = start.clone(); 
        
        if he == 1 {
            continue;
        }
        while temp > BigInt::one() {
            // println!("{:?}.{:?}",temp,he);
            if &temp % &he == BigInt::zero() {
                temp /= he; 
            } else {
                break;
            }
        }
        
        if temp == BigInt::one() {
            println!("{:?}", start); 
            count += 1;
        }
    }
    
    count
}

pub fn euler109() -> i32 {
    let mut count = 0;
    let mut start:u64 = 80;
    let mut h:HashMap<u64,HashSet<u64>> = HashMap::new();

    for i in 11..99{
        let mut num = i;
        h.entry(i).or_insert_with(HashSet::new);
        for j in 3..10{
            num = num * i;
            if let Some(set) = h.get_mut(&i) {
                set.insert(num);
            }
        }
    }
    while count < 30 {
        start += 1;
        if (start % 10 == 9) || (start % 10 == 0) || (start % 10 == 7){
            continue;
        }
        let he = sum_digits(start); 
        
        let mut temp = start.clone(); 
        
        if he == 1 {
            continue;
        }
        if let Some(set1) = h.get(&he) { // 使用 h.get 而不是 h.get_mut
            while temp > 1 {
                if set1.contains(&temp) {
                    println!("{:?},{:?}", start, he);
                    count += 1;
                    break;
                }
                if temp % he == 0 {
                    temp /= he;
                } else {
                    break;
                }
            }

            if temp == 1 {
                println!("{:?},{:?}", start, he);
                count += 1;
            }
        }
    }
    
    count
}

use std::collections::BTreeSet;

fn digit_sum(num: &BigUint) -> u64 {
    let mut sum = 0u64;
    let mut n = num.clone();
    let ten = BigUint::from(10u64);
    while !n.is_zero() {
        let digit = &n % &ten;
        sum += digit.to_u64_digits().get(0).copied().unwrap_or(0);
        n = n / &ten;
    }
    sum
}

fn number_of_digits(num: &BigUint) -> usize {
    let mut count = 0;
    let mut n = num.clone();
    let ten = BigUint::from(10u64);
    while !n.is_zero() {
        count += 1;
        n = n / &ten;
    }
    count
}

pub fn euler109_c() -> BigUint{
    let mut candidates = BTreeSet::new();
    // 增大k的范围以确保足够的候选数
    for k in 2..=50 {
        let mut s = 2u64;
        loop {
            let num = BigUint::from(s).pow(k);
            if num < BigUint::from(10u64) {
                s += 1;
                continue;
            }
            let sum = digit_sum(&num);
            if sum == s {
                candidates.insert(num.clone());
            }
            let digits = number_of_digits(&num);
            if 9 * digits < s as usize {
                break;
            }
            s += 1;
        }
    }
    let sorted: Vec<_> = candidates.into_iter().collect();
    // 确保有足够的候选数
    if sorted.len() >= 30 {
        println!("a_30 = {}", sorted[29]);
        return sorted[29].clone();;
    } else {
        println!("Found only {} candidates. Try increasing the k range.", sorted.len());
        return sorted[0].clone();;
    }
}