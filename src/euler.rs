use std::collections::{HashSet, VecDeque, HashMap};
extern crate rand;

extern crate num_bigint;
extern crate num_traits;
extern crate bit_set;
use std::collections::BinaryHeap;
use std::cmp::Reverse;


use num::integer::Roots;
use num_bigint::BigUint;
use num_traits::{One, Pow};
use bit_set::BitSet;
use rand::distributions::uniform::SampleBorrow;
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
// fn backtrack31(a:Vec<i32>,target:i32)->Vec<i32>{
//     let cointype = [100,50,20,10,5,2,1];
//     if target == 0{
//         return a.sort_unstable();
//     }
//     for i in cointype.iter(){
//         if target - i >= 0{
//             a.push(i);
//             backtrack31(a,target-i);
//         }
//     }

// }
// pub fn euler31()->i32{
//     let cointype = [100,50,20,10,5,2,1];
//     let mut allcoins = HashSet::new();
//     let mut res = 1;
//     let mut coin = Vec::new();
//     for i in &cointype{
//         let mut target = 200;
//         coin.push(&i);
//         backtrack31(coin,200-i);
//         coin.pop(&i);

//     }

//     allcoins.len()
// }

pub fn euler31_dp() -> i32 {
    let cointypes = [1,2, 5, 10, 20, 50, 100];  // 可用的硬币种类
    let mut dp = vec![0; 201];  // dp[i] 存储凑成 i 分的不同方式
    dp[0] = 1;  

    for &coin in cointypes.iter() {
        for i in coin..=200 {
            dp[i as usize] += dp[(i - coin) as usize];
            println!("{:?},{:?}",i,dp[i]);
        }
    }
    dp[200]+1
}
//too big to calculate
fn dp78(n:i32)->i32{
    if n == 1{
        return 1;
    }
    if n == 2{
        return 2;
    }
    let mut res = 0;
    for i in 1..n/2+1{
        res += 1;
    }
    res+dp78(n-1)
}
const MOD: i64 = 500500507;
const LIMIT: usize = 500;

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

pub fn euler500() -> i64 {
    let mut heap = BinaryHeap::new();
    let mut num = 1;
    let mut next_prime = 2;

    for _ in 0..LIMIT {
        heap.push(Reverse(next_prime));
        next_prime += 1;
        
        while !is_prime(next_prime) {
            next_prime += 1;
        }
    }
    for i in 0..LIMIT {
        let Reverse(smallest) = heap.pop().unwrap();
        num = (num * mod_pow(smallest, 1, MOD)) % MOD;
        println!("{:?},{:?},{:?}",i,smallest,num);
        heap.push(Reverse((smallest as i64).pow(2)));
    }

    num
}

pub fn euler81()->i32{
    let s = fs::read_to_string("source/0081_matrix.txt").expect("Failed to read the file");
    let nums:Vec<_> = s.split('\n').collect();
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; 80]; 80];
    for &a in nums.iter(){
        let row:Vec<i32> = a.split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if !row.is_empty() {
        // println!("{:?}",row);
        matrix.push(row);
    }
    }

    for i in 0..dp.len(){
        for j in 0..dp.len(){
            if i == 0 && j == 0{
                dp[i][j] = matrix[i][j];
            }
            else if i == 0{
                dp[i][j] = dp[i][j-1] + matrix[i][j];
            }
            else if j == 0{
                dp[i][j] = dp[i-1][j] + matrix[i][j];
            }
            else{
                dp[i][j] = dp[i-1][j].min(dp[i][j-1]) + matrix[i][j];
            }
        }
    }
    dp[79][79]
}

pub fn euler20(n:i32)->i32{
    let mut sumall = 0;
    let mut res = vec![1];

    for i in 2..n+1{
        let mut carry = 0;
        for num in res.iter_mut(){
            let temp = *num* i  +carry;
            *num = temp%10;
            carry = temp/10;
        }
        while  carry > 0{
            res.push(carry%10);
            carry = carry/10;
        }
    }
    for num in res.iter(){
        sumall += num;
    }
    sumall
}
    
pub fn euler11() -> i32 {
    let s = fs::read_to_string("source/11.txt").expect("Failed to read the file");
    let matrix: Vec<Vec<i32>> = s
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut res = 0;

    // Iterate through the matrix
    let n: i32 = matrix.len() as i32;
    let m: i32 = matrix[0].len() as i32;
    for i in 0..n {
        for j in 0..m {
            // Compute products for all directions that are possible from (i, j)
            let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];
            for (dx, dy) in directions.iter() {
                if i as i32 + 3 * dx < n as i32 && i as i32 + 3 * dx >= 0 && j as i32 + 3 * dy < m as i32&& j as i32 + (3 * dy) >= 0 {
                    let mut product = 1;
                    for k in 0..4 {
                        product *= matrix[(i + k * dx) as usize][(j + k  * dy ) as usize];
                    }
                    res = res.max(product);
                }
            }
        }
    }

    res
}
    

fn dp15(x:i32,y:i32)->i32{
    if x == 0 || y == 0 {
        return 1;
    }
    if x == 1&& y == 1{
        return 2;
    }

    return dp15(x-1,y) + dp15(x,y-1);
    
}


pub fn euler15(n:usize)->i64{
    let mut dp:Vec<Vec<i64>> = vec![vec![1; n+1]; n+1];
    for i in 1..n+1{
        for j in 1..n+1{
            dp[i][j] = dp[i-1][j] + dp[i][j-1];
        }
    }
    dp[n][n]

}

pub fn euler18()->i32{
    let s = fs::read_to_string("source/18.txt").expect("Failed to read the file");
    let matrix: Vec<Vec<i32>> = s.lines()
    .map(|line| {
        line.split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect::<Vec<i32>>()
    })
    .collect();

    let m = matrix.len();
    let n = matrix[m-1].len();

    let mut res: Vec<Vec<i32>> = vec![vec![0;n];m];
    res[0][0] = matrix[0][0];
    for i in 1..m{
        for j in 0..i+1{
            if j == 0{
                res[i][j] = res[i-1][j] + matrix[i][j];
            }else if j == i{
                res[i][j] = res[i-1][j-1] + matrix[i][j];
            }else{
                res[i][j] = res[i-1][j].max(res[i-1][j-1]) + matrix[i][j];
            }
        }
    }
    *res.last().unwrap().iter().max().unwrap()
}

pub fn euler19()->i32{
    let mut day_of_week = 1; // 1 Jan 1900 was a Monday
    let mut sunday_count = 0;

    day_of_week = 366%7;

    println!("{:?}",day_of_week);
    for year in 1901..=2000 {
        for &month in &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31] {
            if day_of_week == 0 {
                sunday_count += 1;
            }
            day_of_week = (day_of_week + month) % 7;
            if month == 28 && ((year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)) {
                day_of_week = (day_of_week + 1) % 7;
            }
        }
    }
    sunday_count
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

pub fn euler24() -> String {
    let mut digits: Vec<char> = "0123456789".chars().collect();
    let mut remaining = 1_000_000 - 1;  
    let mut result: String = String::new();

    for i in (1..10).rev() {
        let f = factorial(i);
        
        let index = remaining / f;
        remaining %= f;
        result.push(digits.remove(index));
    }
    result.push(digits[0]);  

    result
}

const MODULO: i64 = 10_000_000_000;  // 10^10

pub fn euler48(n: i64) -> i64 {
    let mut res = 0;
    for i in 1..=n {
        let mut term = 1;
        for _ in 0..i {
            term = (term * i) % MODULO;
        }
        res = (res + term) % MODULO;
    }
    res
}

pub fn euler30() -> i32 {
    let mut total_sum = 0;
    for num in 10..=354_294 {
        let mut sum = 0;
        let mut temp = num;
        while temp > 0 {
            let digit = temp % 10;
            sum += digit.pow(5u32);
            temp /= 10;
        }
        if sum == num {
            total_sum += num;
        }
    }
    total_sum
}

pub fn euler28(n: i32) -> i32 {
    let mut sum = 1;  
    let mut current = 1;

    for step in 3..=n {
        if step % 2 == 0 {
            continue;
        }
        
        for _ in 0..4 {
            current += step - 1;
            println!("{:?}",current);
            sum += current;
        }
    }

    sum
}

pub fn euler29b(a_limit: u32, b_limit: u32) -> usize {
    let mut distinct_terms = HashSet::new();

    for a in 2..=a_limit {
        for b in 2..=b_limit {
            let mut base = a;
            let mut exponent = b;

            let mut factor = 2;
            while factor * factor <= base {
                if base % factor == 0 {
                    let mut count = 0;
                    while base % factor == 0 {
                        count += 1;
                        base /= factor;
                    }
                    if base == 1{
                        exponent *= count;
                        base *= factor;
                        // distinct_terms.insert((base,exponent));
                        break;
                    }

                }
                factor += 1;
            }

            let term = (base, exponent);
            distinct_terms.insert(term);
            // println!("{:?}",distinct_terms);
        }
    }

    let mut vec: Vec<(u32, u32)> = distinct_terms.into_iter().collect();
    vec.sort_by(|a, b| {
        a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))
    });
    println!("{:?}",vec);
    vec.len()
}

pub fn euler29a(a_limit: u32, b_limit: u32) -> usize {
    let mut distinct_terms = HashSet::new();

    for a in 2..=a_limit {
        for b in 2..=b_limit {
            let mut base = a;
            let mut term_factors = HashMap::new();

            // Simplify the terms, e.g., 4^3 becomes 2^6
            let mut factor = 2;
            while factor * factor <= base {
                if base % factor == 0 {
                    let mut count = 0;
                    while base % factor == 0 {
                        count += 1;
                        base /= factor;
                    }
                    term_factors.insert(factor, count * b);
                }
                factor += 1;
            }

            if base > 1 {
                term_factors.insert(base, b);
            }

            let mut term_vec: Vec<(u32, u32)> = term_factors.into_iter().collect();
            term_vec.sort();
            
            distinct_terms.insert(term_vec);
        }
    }
    println!("{:?}",distinct_terms);
    distinct_terms.len()
}

pub fn euler23()->i32{
    let mut abundants = HashSet::new();
    let abundantlimit = 28123;
    let mut res = 0;

    for num in 12..abundantlimit+1{
        let mut temp = 1;
        for i in 2..num.sqrt()+1{
            if num%i == 0{
                temp += i;
                if i*i != num{
                    temp += num/i;
                }
           }

        }
        if temp > num{
            abundants.insert(num);
        }
    }
    for i in 1..abundantlimit+1{
        for (j, num) in abundants.iter().enumerate(){
            if i > *num && abundants.contains(&(i - num))
            {
                break;
            }
            if j == abundants.len()-1{
                res += i;
            }
    }
}

// Find numbers that can't be written as the sum of two abundant numbers
// for i in 1..=abundantlimit {
//     if !abundants
//         .iter()
//         .any(|&abundant| abundants.contains(&(i - abundant)))
//     {
//         res += i;
//     }
// }
    // let mut a :Vec<i32> = abundants.into_iter().collect();
    // a.sort();
    // println!("{:?}",a);
    res

}

pub fn euler67()->i32{
    let s = fs::read_to_string("source/67.txt").expect("Failed to read the file");
    let matrix: Vec<Vec<i32>> = s.lines()
    .map(|line| {
        line.split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect::<Vec<i32>>()
    })
    .collect();

    let m = matrix.len();
    let n = matrix[m-1].len();

    let mut res: Vec<Vec<i32>> = vec![vec![0;n];m];
    res[0][0] = matrix[0][0];
    for i in 1..m{
        for j in 0..i+1{
            if j == 0{
                res[i][j] = res[i-1][j] + matrix[i][j];
            }else if j == i{
                res[i][j] = res[i-1][j-1] + matrix[i][j];
            }else{
                res[i][j] = res[i-1][j].max(res[i-1][j-1]) + matrix[i][j];
            }
        }
    }
    *res.last().unwrap().iter().max().unwrap()
}

fn factorial34(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        (1..=n).product()
    }
}

pub fn euler34() -> u32 {
    let factorials: Vec<u32> = (0..=9).map(factorial34).collect();
    let mut sum = 0;

    // The upper bound is 7 * 9! = 2540160
    for n in 10..factorial34(9) {
        let mut temp = n;
        let mut factorial_sum = 0;

        while temp > 0 {
            factorial_sum += factorials[(temp % 10) as usize];
            temp /= 10;
        }

        if factorial_sum == n {
            sum += n;
            println!("{:?}",n);
        }
    }

    sum
}

fn is_binary_palindrome(mut n: u32) -> bool {
    let mut binary_str = String::new();

    while n > 0 {
        binary_str.push_str(&(n % 2).to_string());
        n /= 2;
    }

    let reversed_str: String = binary_str.chars().rev().collect();
    binary_str == reversed_str
}
pub fn euler36(limit: u32) -> u32 {
    
    let mut i = 1;
    let mut res = 0;
    while i < limit.sqrt() {  // 调整这个上限以生成更大或更小的回文数
        let s = i.to_string();
        let rs = s.chars().rev().collect::<String>();

        // 构造奇数长度的回文数
        let odd_palindrome: u32 = format!("{}{}", s, &rs[1..]).parse().unwrap();
        if odd_palindrome < limit && is_binary_palindrome(odd_palindrome){
            res += odd_palindrome;
            println!("{:?},{:?}",i,odd_palindrome);
        }
        
        // 构造偶数长度的回文数
        let even_palindrome: u32 = format!("{}{}", s, rs).parse().unwrap();
        if even_palindrome < limit && is_binary_palindrome(even_palindrome){
            res += even_palindrome;
            println!("{:?},{:?}",i,even_palindrome);
        }
        
        i += 1;
    }
    res
    // palindromes.sort();
    // palindromes
}

fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

// pub fn euler36n(n:i32) {
//     let mut palindromes: Vec<i32> = Vec::new();

//     for i in 1..n {
//         if is_palindrome(&i.to_string()) {
//             palindromes.push(i);
//         }
//     }
//     println!("Generated palindromes: {:?}", palindromes);
// }
pub fn euler50(n:i64)->i64{
    let primes = generate_primes(n);
    let mut sum = 0;
    let mut res = 0;
    let mut max_length = 0;

    for start in 0..primes.len() {
        let mut sum = 0;
        for end in start..primes.len() {
            sum += primes[end];
            if sum > n {
                break;
            }
            let length = end - start + 1;
            if length > max_length && primes.contains(&sum) {
                println!("{:?},{:?},{:?}",start,end,length);
                max_length = length;
                res = sum;
            }
        }
    }
    res
}

pub fn euler38()->i32{
    let mut big = 10000;
    let mut res = 0;
    for i in 2..big{
        let mut s = i.to_string();
        let mut j = 2;
        while s.len() < 9 && !s.contains('0'){
            let mut one = i*j;
            s.push_str(&one.to_string());
            j+= 1;
        }
        if !s.contains('0') &&s.len() == 9 && s.chars().collect::<std::collections::HashSet<_>>().len() == s.len(){
            
            res = res.max(s.parse().unwrap());
            // println!("{:?},{:?}",i,res);
        }
    }
    res
}
pub fn euler38c() -> i32 {
    let mut max_pandigital = 0;

    for i in 1..10000 {
        let mut s = String::new();
        let mut j = 1;
        while s.len() < 9 {
            s.push_str(&(i * j).to_string());
            j += 1;
        }
        if s.len() == 9 
            && !s.contains('0') 
            && s.chars().collect::<std::collections::HashSet<_>>().len() == 9 {
            let num = s.parse::<i32>().unwrap();
            max_pandigital = std::cmp::max(max_pandigital, num);
        }
    }
    max_pandigital
}
fn sort_digits_of_number(number: i32) -> Vec<char> {
    let mut digits: Vec<char> = number.to_string().chars().collect();
    digits.sort();
    digits
    // sorted_number.parse::<i32>().unwrap()
}

pub fn euler52()->i32{
    for i in 100000..200000{
        let s = sort_digits_of_number(i);
        // println!("{:?}",s);
        for j in 2..7{
            if s != sort_digits_of_number(i * j){
                break;
            }
            if j == 6{
                return i;
            }
        }
        
    }
    0
}

fn sort_digits_of_numberc(number: i32) -> Vec<char> {
    let mut digits: Vec<char> = number.to_string().chars().collect();
    digits.sort();
    digits
}

pub fn euler52c()->i32{
    let mut x: i32 = 100000;
    loop {
        let x_digits = sort_digits_of_numberc(x);
        if (2..=6).all(|n| sort_digits_of_numberc(n * x) == x_digits) {
            return x;      
        }
        x += 1;
    }
    0
}
pub fn euler39(p:i32)->i32{
    let mut res = HashMap::new();
    for a in 1..p/3{
        for b in a..p/2{
            for c in b..p/2{
                if a*a + (b*b) == c*c{
                    *res.entry(a+b+c).or_insert(0) += 1;
                    if a + b + c == 840{
                        println!("{:?},{:?},{:?}",a,b,c);
                    }
                }
            }
        }
    }
    let max_key = res.into_iter().max_by_key(|&(_, v)| v).map(|(k, _)| k);
    max_key.unwrap()
}

pub fn euler39c(p:i32) ->i32{
    let mut count = HashMap::new();
    let limit = p;

    for a in 1..limit/3 {
        for b in a..limit/2 {
            let c = (a * a + b * b) as f64;
            let c = c.sqrt();

            if c == c.floor() {
                let p = a + b + c as i32;
                if p <= limit {
                    *count.entry(p).or_insert(0) += 1;
                }
            }
        }
    }

    let max_p = count.into_iter().max_by_key(|&(_, v)| v).map(|(k, _)| k);
    max_p.unwrap()
}

pub fn euler32()->i32{
    let mut res: HashSet<i32> = HashSet::new();

    for i in 11..100{
        for j in 100..1000{
            let mut s = i.to_string();
            s.push_str(&(j.to_string()));
            s.push_str(&(i*j).to_string());
            if !s.contains('0') && s.len() == 9 && s.chars().collect::<std::collections::HashSet<_>>().len() == s.len(){
                println!("{:?}",s);
                res.insert(i*j);
            }
        }
    }
    for i in 2..10{
        for j in 1000..10000{
            let mut s = i.to_string();
            s.push_str(&(j.to_string()));
            s.push_str(&(i*j).to_string());
            if !s.contains('0') && s.len() == 9 && s.chars().collect::<std::collections::HashSet<_>>().len() == s.len(){
                println!("{:?}",s);
                res.insert(i*j);
            }
        }
    }
    res.iter().sum()
}

pub fn euler46()->i64{
    let primes = generate_primes(1000000);
    let mut i = 33;
    while i < 100000000{
        let mut res = true;
        for prime in primes.iter(){
            let mut temp =  i - prime;
            // println!("{:?}",temp);
            if temp < 0{
                break;
            }
            temp = temp/2;
            let temp2 = temp.sqrt();
            if temp2*temp2 == temp{
                res = false;
                break;
            }
        }
        if res{
            return i;
        }
        i += 2;
    }
0
}
pub fn euler46c() -> i64 {
    let mut n = 9;
    loop {
        // Check if n is a composite number
        if !is_prime(n) {
            let mut conjecture_holds = false;
            let limit = ((n / 2) as f64).sqrt() as i64;
            
            // Check if the conjecture holds for this n
            for s in 1..=limit {
                let remainder = n - 2 * s * s;
                if is_prime(remainder) {
                    conjecture_holds = true;
                    break;
                }
            }
            
            // If the conjecture doesn't hold, we've found our answer
            if !conjecture_holds {
                return n;
            }
        }
            n += 2;
    }
}

pub fn euler37()->i64{
    let primes = generate_primes(1000000);
    let mut res = Vec::new();
    for &prime in primes.iter(){
        if prime < 10{
            continue;
        }
        // let mut num = prime;
        let mut mult = 10;
        while mult < prime{
            if !is_prime(prime%mult) || !is_prime(prime/mult){
                break;
            }
            mult = mult * 10;

        }
        if mult > prime{
            res.push(prime);
            if res.len()>12{
                break;
            }
        }

    }
    println!("{:?}",res);
    res.iter().sum()
}

pub fn euler42()->usize{
    let mut triangle: Vec<i32> = Vec::new();
    // let mut count: HashSet<i32> = HashSet::new();
    let mut count = 0;
    for i in 1..100{
        triangle.push(i*(i+1)/2);
    }
    println!("{:?}",triangle);
    let content = fs::read_to_string("source/42.txt").expect("Failed to read the file");
    let names:Vec<String> = content.split(",").map(|part|part.replace("\"", "")).collect();

    for name in names.iter(){
        let num:Vec<i32> = name.chars().map(|x|char_to_number(x)).collect();
        let value:i32 = num.iter().sum();
        if triangle.contains(&value){
            // count.insert(value);
            count += 1;
        }
    }
    count
}

fn triangle_number(n: u32) -> u32 {
    n * (n + 1) / 2
}

// Convert a word to its word value
fn word_value(word: &str) -> u32 {
    word.chars().map(|ch| (ch as u32) - 64).sum()
}

// Check if a number is a triangle number
fn is_triangle_number(num: u32) -> bool {
    let mut n = 1;
    let mut t = triangle_number(n);
    while t < num {
        n += 1;
        t = triangle_number(n);
    }
    t == num
}

fn euler42c()->usize{
    let data = fs::read_to_string("42.txt").expect("Unable to read file");
    let words: Vec<&str> = data.split(',').map(|s| s.trim_matches('"')).collect();

    let count = words.iter().filter(|&&word| is_triangle_number(word_value(word))).count();
    count
}

pub fn euler44()->i32{
    // let mut triangle: Vec<i32> = Vec::new();
    let mut triangle: HashSet<i32> = HashSet::new();
    let mut res = i32::MAX;
    for i in 1..3000{
        // triangle.push(i*(3*i-1)/2);
        triangle.insert(i*(3*i-1)/2);
    }
    println!("{:?}",triangle);
    // for i in 0..triangle.len(){
    //     for j in i..triangle.len(){
    //         if triangle.contains(&(triangle[j] + triangle[i])) && triangle.contains(&(triangle[j]-triangle[i])){
    //             println!("{:?},{:?}",triangle[i],triangle[j]);
    //             res = res.min(triangle[j]-triangle[i]);
    //         }
    //     }
    // }
    for i in 1..triangle.len(){
        for j in i..triangle.len(){
            let a: i32 = i as i32*(3*i as i32-1)/2;
            let b:i32 = j as i32 *(3*j as i32-1)/2;
            if triangle.contains(&(a+b)) && triangle.contains(&(b-a)){
                 println!("{:?},{:?}",a,b);
                res = res.min(b-a);
            }
        }
    }
    res

}

pub fn euler45()->i64{
    let mut matrix: HashMap<i64, usize> = HashMap::new();
    let mut n:i64 = 144;
    loop{
        let mut num = n*(n+1)/2;
        let counter = matrix.entry(num).or_insert(0);
        *counter += 1;
        if *counter == 3{
            return num;
        }

        let mut num = n*(3*n-1)/2;
        let counter = matrix.entry(num).or_insert(0);
        *counter += 1;
        if *counter == 3{
            return num;
        }

        let mut num = n*(2*n-1);
        let counter = matrix.entry(num).or_insert(0);
        *counter += 1;
        if *counter == 3{
            return num;
        }
        // println!("{:?}",matrix);
        n += 1;
    }
}
pub fn euler45_quick() -> i64 {
    let mut pentagonals = HashSet::new();
    let mut n = 144; // 因为题目要求 n > 143

    loop {
        let hexagonal = n * (2 * n - 1);
        if !pentagonals.insert(hexagonal){
            return hexagonal;
        }
        let pentagonal = n * (3 * n - 1) / 2;
            if !pentagonals.insert(pentagonal){
                return pentagonal;
            }
        n += 1;
    }
}

