use std::collections::{HashSet, VecDeque, HashMap};
extern crate rand;

extern crate num_bigint;
extern crate num_traits;
extern crate bit_set;


use num::integer::Roots;
use num_bigint::BigUint;
use num_traits::{One, Pow};
use bit_set::BitSet;
use std::fs;

const SEGMENT_SIZE: usize = 1_000_000;
const MO:i64 = 1_000_000_007;
//is there a better way?
/// 2,interesting , you can get 4 F(n-3) + F(n-6) for even-valued
fn euler2_f(n:i64) -> i64{
    if n == 1{
        return 1;
    }
    if n == 2{
        return 2;
    }
    
    return euler2_f(n-1) + euler2_f(n-2);

}
pub fn euler2_recursion(n:i64) -> i64{
    let mut res = 0;
    let mut i = 0;
    loop{
        let eu = euler2_f(i);
        println!("{:?}",eu);
        if eu > n{
            break;
        }
        res += eu;
        i += 1;
    }
    return res;
}
pub fn euler2(n:i64)->i64{
    let mut one = 1;
    let mut two = 1;
    let mut target = 0;
    let mut res = 0;

    loop{
        target = one + two;
        println!("{:?},{:?}",target,n);
        if target > n{
            break;
        }
        two = one;
        one = target;
        if target %2 == 0{
            res += target;
        }
        
    }
    return res;
}


//euler6
pub fn euler6(n:i64)->i64{
    let mut res:i64 = 0;
    for i in 1..n{
        for j in i+1..n+1{
            res += i * j *2;
        }
    }
    return res;
}

fn generate_primes(n: i64) -> Vec<i64> {
    let mut is_prime = vec![true; n as usize];
    let mut primes = Vec::new();

    for i in 2..n {
        if is_prime[i as usize] {
            primes.push(i);
            let mut j = 2 * i;
            while j < n {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }

    primes
}

pub fn euler5(n:i64)->i64{
    let primes = generate_primes(n);
    println!("{:?}",primes);
    let mut product = primes.iter().product();

    let mut j: u32 = 2;
    for i in primes.iter(){
        j = 2;
        while i.pow(j) <= n{
            println!("{:?}",product);
            product *= i;
            j += 1;
        }
    }
    product
}

fn is_prime(num: i64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as i64) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub fn euler7(n:i32) -> i64{
    let mut primes_found = 0;
    let mut num = 2;

    while primes_found < n {
        if is_prime(num) {
            primes_found += 1;
            println!("{:?}",num);
        }

        if primes_found < n {
            num += 1;
        }
    }
    
    num
}
// pub fn euler8(n:usize)->i32{
//     let s = "73167176531330624919225119674426574742355349194934
//     96983520312774506326239578318016984801869478851843
//     85861560789112949495459501737958331952853208805511
//     12540698747158523863050715693290963295227443043557
//     66896648950445244523161731856403098711121722383113
//     62229893423380308135336276614282806444486645238749
//     30358907296290491560440772390713810515859307960866
//     70172427121883998797908792274921901699720888093776
//     65727333001053367881220235421809751254540594752243
//     52584907711670556013604839586446706324415722155397
//     53697817977846174064955149290862569321978468622482
//     83972241375657056057490261407972968652414535100474
//     82166370484403199890008895243450658541227588666881
//     16427171479924442928230863465674813919123162824586
//     17866458359124566529476545682848912883142607690042
//     24219022671055626321111109370544217506941658960408
//     07198403850962455444362981230987879927244284909188
//     84580156166097919133875499200524063689912560717606
//     05886116467109405077541002256983155200055935729725
//     71636269561882670428252483600823257530420752963450";
//     let s = s.replace("\n", "").replace(" ", "");
//     let mut res = i32::MIN;
//     let mut temp = 1;
//     let length = s.len();

//     let mut qu = VecDeque::new();
//     let mut i = 0;
//     while i < length{
//         let num = s[i].to_digit(10);
//         if num == 0{
//             qu.clear();
//             temp = 1;
//             i += 1;
//             continue;

//         }
//         if qu.len() < n {
//             qu.push_back(num);
//             temp *= num;
//             i += 1;
//             continue;
//         }
//         res = res.max(temp);
//         temp = temp*num/qu.pop_front();
//         res = res.max(temp);
//         i += 1;
//     }
//     res
// }

pub fn euler8(n: usize) -> i64 {
    let s = "73167176531330624919225119674426574742355349194934
//     96983520312774506326239578318016984801869478851843
//     85861560789112949495459501737958331952853208805511
//     12540698747158523863050715693290963295227443043557
//     66896648950445244523161731856403098711121722383113
//     62229893423380308135336276614282806444486645238749
//     30358907296290491560440772390713810515859307960866
//     70172427121883998797908792274921901699720888093776
//     65727333001053367881220235421809751254540594752243
//     52584907711670556013604839586446706324415722155397
//     53697817977846174064955149290862569321978468622482
//     83972241375657056057490261407972968652414535100474
//     82166370484403199890008895243450658541227588666881
//     16427171479924442928230863465674813919123162824586
//     17866458359124566529476545682848912883142607690042
//     24219022671055626321111109370544217506941658960408
//     07198403850962455444362981230987879927244284909188
//     84580156166097919133875499200524063689912560717606
//     05886116467109405077541002256983155200055935729725
//     71636269561882670428252483600823257530420752963450";
    let s = s.replace("\n", "").replace(" ", ""); // remove whitespaces
    let mut res = i64::MIN;
    let mut temp:i64 = 1;
    let mut qu = VecDeque::new();


    let mut i = 0;
    for c in s.chars() {
        i += 1;
        if let Some(num) = c.to_digit(10) {
            let num = num as i32;
            println!("{:?},{:?},{:?}",i,num,res);
            if num == 0 {
                qu.clear();
                temp = 1;
                continue;
            }

            qu.push_back(num);
            temp *= num as i64;

            if qu.len() > n {
                if let Some(front) = qu.pop_front() {
                    temp /= front as i64;
                }
            }

            if qu.len() == n {
                res = res.max(temp);
            }
        }
    }

    res
}
fn euler9(n:i32)->i32{
    // let mut res = 1;
    for i in 1..n-1{
        for j in i+1..n{
            let k = n-i-j;
            if i*i + (j*j) == k*k{
                println!("{:?},{:?},{:?}",n,i,j);
                return i*j*k;
            }
        }
    }
    -1
}
pub fn euler9_p(n:i32){
    // let mut res = 1;
    for i in 1..n+1{
        euler9(i);
    }
}

pub fn euler10(n:i64)->i64{
    let mut res = 0;
    for i in 2..n+1{
        if is_prime(i){
            res += i;
        }
    }
    res
}

pub fn euler16(n:u32)->u32{
    let two = BigUint::from(2u32);
    let big = two.pow(n);
    println!("{:?}",big);
    let s = big.to_string();
    println!("{:?}",s);
    s.chars().map(|c| c.to_digit(10).unwrap()).sum()
}

fn is_prime_216(num: u64, primes: &BitSet) -> bool {
    if num < 2 {
        return false;
    }
    primes.contains(num as usize)
}

fn generate_primes_216(limit: usize) -> BitSet {
    let mut primes = BitSet::with_capacity(limit);
    for i in 2..limit {
        primes.insert(i);
    }
    let mut p = 2;
    while p * p <= limit {
        if primes.contains(p) {
            let mut i = p * p;
            while i < limit {
                primes.remove(i);
                i += p;
            }
            // println!("{:?}",primes);
        }
        p += 1;
    }
    primes
}


fn sieve(limit: usize) -> Vec<bool> {
    let mut primes = vec![true; limit + 1];
    primes[0] = false;
    primes[1] = false;
    let mut p = 2;
    while p * p <= limit {
        if primes[p] {
            let mut i = p * p;
            while i <= limit {
                primes[i] = false;
                i += p;
            }
        }
        p += 1;
    }
    primes
}
fn is_prime_segmented_216p(n: usize, base_primes: &[bool]) -> bool {
    let mut sieve = vec![true; SEGMENT_SIZE];
    let limit = (SEGMENT_SIZE as f64).sqrt() as usize;

    for p in 2..=limit {
        if base_primes[p] {
            let start = std::cmp::max(2, (n + p - 1) / p) * p - n;
            let mut i = start;
            while i < SEGMENT_SIZE {
                sieve[i] = false;
                i += p;
            }
        }
    }

    let t = n * n * 2 - 1;
    sieve[t - n]
}
pub fn euler216(num:i64)->i32{
    let limit = num;
    // let sieve_limit = (limit as u64 * limit as u64 * 2) as usize;
    let base_primes = sieve((num as f64).sqrt() as usize);
    // let primes = generate_primes_216(sieve_limit);
    let mut count = 0;

    for n in 1..=limit {
        if is_prime_segmented_216p(n as usize, &base_primes) {
            count += 1;
        }
    }
    count
}

fn amicable(a:i32)->i32{
    let mut res = 1;
    let mut b = 1;
    for i in 2..a.sqrt(){
        if a%i == 0{
            b += i;
            b += a/i;
        }
    }
    if a == b{
        return 0;
    }
    for i in 2..b.sqrt(){
        if b%i == 0{
            res += i;
            res += b/i;
        }
    }
    if res == a{
        // println!("{:?},{:?}",res,b);
        res
    }
    else{
        0
    }
}
pub fn euler21(n:i32)->i32{
    let mut res = 0;
        for i in 4..n+1{
            res += amicable(i);
        }
    res
}
fn char_to_number(c: char) -> i32 {
    (c.to_ascii_lowercase() as i32) - ('a' as i32) + 1
}
fn process_string(s: &str) -> Vec<String> {
    s.split(',')
        .map(|part| part.replace("\"", "").replace(" ", "").replace("\n", ""))
        .map(|cleaned| cleaned.to_string())
        .collect()
}
pub fn euler22() ->i32{

    let content = fs::read_to_string("source/names.txt").expect("Failed to read the file");
    
    let mut names: Vec<String> = process_string(&content);
    names.sort_unstable();
    // println!("{:?}",names);
    let mut res = 0;
    let mut i = 1;
    for n in &names {
        let numbers: Vec<i32> = n.chars().map(|c| char_to_number(c)).collect();
        let value:i32 = numbers.iter().filter(|&&x| x > 0).sum();

        res += value * i;
        i += 1;
    }

    res
}

fn reverse_number(n: i64) -> i64 {
    let s: String = n.to_string();  // Convert the number to a string
    let reversed_s: String = s.chars().rev().collect();  // Reverse the string
    let reversed_n: i64 = reversed_s.parse().unwrap();  // Convert the reversed string back to a number
    reversed_n
}

pub fn euler808(n:i64)->i64{
    let mut res:i64 = 0;
    let primes = generate_primes(100000000);
    let mut reversible = HashSet::new();
    for i in &primes{
        let square:i64 = i * i;
        let reverse:i64 = reverse_number(square);
        if square != reverse{
            let temp:i64 = reverse.sqrt();
            if temp*temp == reverse && primes.contains(&temp) && !reversible.contains(&reverse){
                reversible.insert(square);
                reversible.insert(reverse);
                // println!("{:?}",reversible);
                // println!("{:?},{:?}",square,reverse);
                res += square + reverse ;
                if reversible.len() == n as usize{
                    break;
                }
            }
        }
        
    }    res
}

fn f759(n:i64)->i64{
    let mut i = n;
    if i == 1{
        return 1;
    }
    if i % 2 == 0{
        return 2*f759(i/2);
    }else{
        let j = i/2;
        return f759(j)/j + (2*f759(j)) + i;
    }
}
fn s759(n:i64)->i64{
    let mut res:i64 = 0;
    for i in 1..n+1{
        println!("{:?},{:?},{:?},{:?}",i,f759(i),f759(i)/i,format!("{:b}", i));
        let num = f759(i)%MO;
        res += num*num;
    }
    res%MO
}
fn f759_fast(n: i64) -> i64 {
    let count_ones = (n as u64).count_ones() as i64;
    (count_ones * n) % MO
}

fn s759_fast(n: i64) -> i64 {
    let mut res: i64 = 0;
    for i in 1..=n {
        let num = f759_fast(i);
        res = (res + (num * num)) % MO;
    }
    res % MO
}

pub fn euler759(n:i64)->i64{
    s759_fast(n)
}

pub fn euler700()->i64{
    let first: i128 = 1504170715041707;
    let modone:i128 = 4503599627370517;
    let mut res: i128 = 0;
    let mut minnum = first;
    for i in 2..modone {
        let term = (first * i as i128) % modone;
        if term < minnum {
            minnum = term;
            res += term;
        }
    }
    res as i64
}


pub fn euler14(longth: i64) -> i64 {
    let mut mid = HashMap::new();
    let mut res = 0;
    let mut maxcount = 0;

    for a in 2..longth {
        let mut i = a;
        let mut count = 0;
        while i != 1 && !mid.contains_key(&i) {
            count += 1;
            if i % 2 == 0 {
                i = i / 2;
            } else {
                i = i * 3 + 1;
            }
        }

        if let Some(value) = mid.get(&i) {
            count += *value;
        }

        mid.insert(a, count);

        if maxcount < count {
            maxcount = count;
            res = a;
        }
    }
    // println!("{:?}",mid);
    res
}

pub fn euler14_fast(longth: i64) -> i64 {
    let mut mid = HashMap::new();
    let mut res = 0;
    let mut maxcount = 0;

    for a in 2..longth {
        let mut i = a;
        let mut count = 0;
        while i != 1 {
            if let Some(&cached_count) = mid.get(&i) {
                count += cached_count;
                break;
            }
            count += 1;
            i = if i % 2 == 0 { i / 2 } else { i * 3 + 1 };
        }
        
        // use entry to make it fast
        let total_count = mid.entry(a).or_insert(0);
        *total_count = count;

         
        if maxcount < count {
            maxcount = count;
            res = a;
        }
    }
    res
}

// what the fuck.without hash is faster
pub fn euler14_withnohash(longth: i64) -> i64 {
    let mut res = 0;
    let mut maxcount = 0;

    for a in 10000..longth {
        let mut i = a;
        let mut count = 0;
        while i != 1 {
            count += 1;
            // i = if i % 2 == 0 { i / 2 } else { i * 3 + 1 };
            // use bit the fatest
            i = if i & 1 == 0 { i >> 1 } else { i * 3 + 1 };
        }
        

        if maxcount < count {
            maxcount = count;
            res = a;
        }
    }
    res
}

pub fn euler13()->i64{
    let s = fs::read_to_string("source/13.txt").expect("Failed to read the file");
    let nums:Vec<_> = s.split('\n').collect();
    let mut res:i64 = 0;
    for a in nums.iter(){
        let num_str: String = a.chars().take(11).collect();
        let num: i64 = num_str.parse().unwrap_or(0);
        res += num;
    }
    res
}



