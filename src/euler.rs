use std::collections::{HashSet, VecDeque};
extern crate rand;

extern crate num_bigint;
extern crate num_traits;
extern crate bit_set;


use num::integer::Roots;
use num_bigint::BigUint;
use num_traits::{One, Pow};
use bit_set::BitSet;


const SEGMENT_SIZE: usize = 1_000_000;
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

fn generate_primes(n: i32) -> Vec<i32> {
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

pub fn euler5(n:i32)->i32{
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
        println!("{:?},{:?}",res,b);
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