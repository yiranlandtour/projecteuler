
mod util;
mod euler;
extern crate rand;
// extern crate image;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::{Duration, Instant};
pub struct ListNode{
    pub val:i32,
    pub next:Option<Box<ListNode>>
}



fn generate_primes(nums: &Vec<i32>, index: usize, current_sum: i32, primes: &mut HashSet<i32>) {
    if index == nums.len() {
        if util::is_prime(current_sum) {
            primes.insert(current_sum);
        }
        return;
    }

    // Try adding the current number
    generate_primes(nums, index + 1, current_sum + nums[index], primes);

    // Try subtracting the current number
    generate_primes(nums, index + 1, current_sum - nums[index], primes);
}
//36进制的数
fn to_base36(num: u32) -> String {
    let base36_chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = String::new();
    let mut n = num;
    while n > 0 {
        let digit = n % 36;
        n /= 36;
        result.push(base36_chars[digit as usize]);
    }
    result.chars().rev().collect::<String>()
}

// 海盗拿糖
fn best_chance(x: i32, n: usize, taken: &mut Vec<i32>, memo: &mut HashMap<(i32, usize), f64>) -> (f64, i32) {
    if n == 0 {
        let max_candies = *taken.iter().max().unwrap();
        let min_candies = *taken.iter().min().unwrap();

        if max_candies == min_candies {
            return (0.5, 0); // 有一半的机会死亡
        }

        if taken[0] == max_candies || taken[0] == min_candies {
            return (0.0, 0);
        }

        return (1.0, 0);
    }

    if let Some(&chance) = memo.get(&(x, n)) {
        return (chance, 0);
    }

    let mut max_chance = 0.0;
    let mut best_candies = 0;
    for i in 1..=(x + 1 - n as i32) {
        taken.push(i);
        let (chance, _) = best_chance(x - i, n - 1, taken, memo);
        if chance > max_chance {
            max_chance = chance;
            best_candies = i;
        }
        taken.pop();
    }

    memo.insert((x, n), max_chance);
    (max_chance, best_candies)
}
//根号
fn cube_root(x: f64) -> f64 {
    let mut y_old = x; 
    let mut y_new = y_old - (y_old.powi(3) - x) / (3.0 * y_old.powi(2));
    println!("{:?}",y_new);
    let epsilon = 1e-10; 

    while (y_new - y_old).abs() > epsilon {
        y_old = y_new;
        y_new = y_old - (y_old.powi(3) - x) / (3.0 * y_old.powi(2));
        println!("{:?},{:?}",y_old,y_new);
    }

    y_new
}
// 8 皇后
fn is_valid(board: &Vec<i32>, row: usize, col: i32) -> bool {
    for i in 0..row {
        if board[i] == col || board[i] - i as i32 == col - row as i32 || board[i] + i as i32 == col + row as i32 {
            return false;
        }
    }
    true
}

fn solve(board: &mut Vec<i32>, row: usize, solutions: &mut Vec<Vec<String>>) {
    if row == board.len() {
        let mut solution = Vec::new();
        for &col in board.iter() {
            let mut chars = vec!['.'; board.len()];
            chars[col as usize] = 'Q';
            solution.push(chars.into_iter().collect());
        }
        solutions.push(solution);
    } else {
        for col in 0..(board.len() as i32) {
            if is_valid(board, row, col) {
                board[row] = col;
                solve(board, row + 1, solutions);
            }
        }
    }
}

fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut solutions = Vec::new();
    let mut board = vec![-1; n as usize];
    solve(&mut board, 0, &mut solutions);
    solutions
}

fn custom_compare(a: &i32, b: &i32) -> std::cmp::Ordering {
    let a_str = a.to_string();
    let b_str = b.to_string();
    
    if a_str.len() == b_str.len() {
        if a_str.starts_with("5") && !b_str.starts_with("5") {
            return std::cmp::Ordering::Greater;
        } else if !a_str.starts_with("5") && b_str.starts_with("5") {
            return std::cmp::Ordering::Less;
        }
    }
    a.cmp(b)
}

//大数相乘，多大的数字都可以
pub fn multiply(num1: String, num2: String) -> String {
    let n1 = num1.len();
    let n2 = num2.len();
    let mut res = vec![0; n1 + n2]; // 最大可能长度为 n1 + n2
    let num1 = num1.bytes().rev().collect::<Vec<_>>();
    let num2 = num2.bytes().rev().collect::<Vec<_>>();
    println!("{:?},{:?}",num1,num2);
    for i in 0..n1 {
        for j in 0..n2 {
            res[i + j] += (num1[i] - b'0') * (num2[j] - b'0');
            if res[i + j] >= 10 {
                res[i + j + 1] += res[i + j] / 10;
                res[i + j] %= 10;
            }
            println!("{:?}",res);
        }
    }
    while res.len() > 1 && res[res.len() - 1] == 0 {
        res.pop();
    }
    res.into_iter().rev().map(|x| (x + b'0') as char).collect()
}
//哥德巴赫
fn is_perfect_square(num: i64) -> bool {
    let root = (num as f64).sqrt().round() as i64;
    root * root == num
}

fn find_xyz(limit: i64) -> Option<(i64, i64, i64)> {
    let limit_cubed = (limit as f64).powf(2.0/3.0).ceil() as i64;
    for x in 1..=limit_cubed {
        let y3 = limit * limit - x * x * x;
        if y3 <= 0 {
            break;
        }
        if is_perfect_square(y3) {
            let y = (y3 as f64).cbrt().round() as i64;
            if x * x * x + y * y * y == limit * limit {
                return Some((x, y, limit));
            }
        }
    }
    None
}

fn distance(a: &[i32; 2], b: &[i32; 2]) -> f64 {
    ((a[0] as f64 - b[0] as f64).powi(2) + (a[1] as f64 - b[1] as f64).powi(2)).sqrt()
}

fn midpoint(a: &[i32; 2], b: &[i32; 2]) -> [f64; 2] {
    [(a[0] as f64 + b[0] as f64) / 2.0, (a[1] as f64 + b[1] as f64) / 2.0]
}

fn max_points_on_circle(points: &[[i32; 2]]) -> usize {
    let mut max_count = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let center = midpoint(&points[i], &points[j]);
            let radius = distance(&points[i], &[center[0] as i32, center[1] as i32]);
            let count = points.iter().filter(|&&p| (distance(&p, &[center[0] as i32, center[1] as i32]) - radius).abs() < 1e-9).count();
            max_count = max_count.max(count);
        }
    }

    max_count
}
// pub fn solve_polynomial(coefficients: Vec<f64>) -> Vec<f64> {
//     let degree = coefficients.len() - 1;
//     let mut companion_matrix = DMatrix::from_element(degree, degree, 0.0);
//     for i in 0..degree {
//         companion_matrix[(i, degree-1)] = -coefficients[i] / coefficients[degree];
//     }
//     for i in 0..degree-1 {
//         companion_matrix[(i+1, i)] = 1.0;
//     }
//     let eigenvalues = companion_matrix.eigenvalues();
//     eigenvalues.into_iter().map(|c| c.re).collect()
// }
fn measure_time<F>(f: F) -> ((), Duration)
where
    F: FnOnce() -> ()
{
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();
    ((), elapsed)
}

fn main() {
    // let n = 10;
    // let (mut a, mut b) = (1,1);
    // (0..n).for_each(|_| {b= a + b;a = b-a});
    // println!("{:?}",a);
    // for i in 20..10{
    //     println!("{:?}",i);
    // }
    // let s = String::from("good");
    // let s = "ass";
    // for(i,v) in s.chars().enumerate(){
    //         println!("{:?}",i);
    //         println!("{:?}",v);
    // }

    // let mut stack = vec!['0'];
    // let s = "(((}}{)){}";
    // for c in s.chars() {
    //     match c {
    //         '(' | '[' | '{' => {stack.push(c)},
    //         ')' => {if stack.pop().unwrap() != '(' {println!("aa") }},
    //         ']' => {if stack.pop().unwrap() != '[' {println!("bb")}},
    //         '}' => {if stack.pop().unwrap() != '{' {println!("cc")}},
    //         _ => (),
    //     }
    // }
    // stack.len() == 1
    // let a = [1,3,5,9,10];
    // let target = 9;
    // b= a.binary_search(&4).unwrap_or_else(|x| x) as i32;
    // println!("{:?}",b);
    // let mut a = vec![];
    // a.push(1);
    // a.push(2);
    // let b = [3,9];
    // let mut bad = false;
    // let mut bad2 = true;
    // for reward in &a{
    //     bad = bad || (*reward == b[0]);
    // }
    // if b.len() > 1 {
    //     bad2 = false;
    //     for reward in &a{
    //         bad2 = bad2 || (*reward == b[1]);
    //     }
    // }
    // bad = bad && bad2;
    // if !bad{
    //     println!("bad");
    // }

    // for c in &b{
    //     a.push(*c);
    // }
    // if let Some(_) =  a.iter().position(|&x| x == 199){
    // println!("asdf");
    // }
    // for i in 0..10{
    //     println!("{:?}",i);
    // }

    // let num1 = "1232828191933";
    // let num2 = "2342342342342";
    // let res = multiply(num1.to_string(), num2.to_string());
    
    // let res = to_base36(72);
    // let solutions = solve_n_queens(12);
    // for (idx, solution) in solutions.iter().enumerate() {
    //     println!("Solution {}:", idx + 1);
    //     for row in solution {
    //         println!("{}", row);
    //     }
    //     println!("");
    // }
    // let x = 100;
    // let n = 3;
    // let mut taken = Vec::new();
    // let mut memo = HashMap::new();
    // let (chance, best_candies) = best_chance(x, n, &mut taken, &mut memo);
    // println!("The best survival chance for the first child is: {}", chance);
    // println!("The first child should take {} candies for the best chance.", best_candies);

    // let x = 100.0;
    // println!("Cube root of {} is approximately {}", x, cube_root(x));

    // if let Some((x, y, n)) = find_xyz(100000000) { // Here 10000 is an arbitrary limit for n.
    //     println!("Found x = {}, y = {}, n = {}", x, y, n);
    // } else {
    //     println!("No solution found within the given limit.");
    // }
    // let points: [[i32; 2];7] = [[0, 0], [1, 0], [0, 1], [1, 1],[0,-1],[2,2],[3,4]];

    // println!("Maximum number of points on a circle: {}", max_points_on_circle(&points));

    // println!("{:?}",res);
    // let nums = vec![2, 4, 51,1,2213,13,111,9];
    // let mut primes = HashSet::new();

    // generate_primes(&nums, 0, 0, &mut primes);

    // let primes_vec: Vec<i32> = primes.into_iter().collect();
    // println!("{:?}", primes_vec);

    // let mut numbers = vec![151, 161, 52, 62, 51, 61];
    // numbers.sort_by(custom_compare);
    // println!("{:?}", numbers);

    //随机的盒子复原
    // let mut rng = rand::thread_rng();
    // let mut n: usize = rng.gen_range(3..10); // 生成2到4的随机数
    // n = 4;
    // let mut nums: Vec<i32> = (1..(n*n)).map(|x| x as i32).collect();
    // nums.push(0);
    // nums.shuffle(&mut rng);

    // println!("{:?}",nums);
    // let mut board = vec![vec![]; n];
    // for i in 0..n {
    //     for _ in 0..n {
    //         board[i].push(nums.pop().unwrap());
    //     }
    // }

    // println!("Generated board:");
    // for row in &board {
    //     println!("{:?}", row);
    // }

    // let result = util::sliding_puzzle_rand(board);
    // println!("\nSteps to solve:");
    // for step in result {
    //     for row in &step {
    //         println!("{:?}", row);
    //     }
    //     println!("------");
    // }

    // let mut nums = vec![5, 10,5];
    // let target = 10;
    // match util::find_expression(&mut nums, target, "".to_string())  {
    //     Some(expr) => println!("{}", expr),
    //     None => println!("No valid expression found"),
    // }

    // let pattern = "abcd+aa=bcbb";
    // let replaced_pattern = pattern.replace(" ", "");
    // if let Some(mapping) = util::find_valid_mapping(&replaced_pattern) {
    //     for (ch, val) in mapping.iter() {
    //         println!("{} -> {}", ch, val);
    //     }
    // } else {
    //     println!("No valid mapping found!");
    // }

    // util::simulate_interaction2();

    // let path = "path_to_your_image.jpg"; // 更改为您的图片路径
    // let ascii_image = util::image_to_ascii(path);
    // for row in ascii_image {
    //     let line: String = row.iter().collect();
    //     println!("{}", line);
    // }

    // let nums = vec![1, -1, 2, -2,9,13,-7,3,-15];
    // let target = 0;
    // let result = util::sum_combinations(nums, target);
    // for combination in &result {
    //     println!("{:?}", combination);
    // }

    // util::rand_go();

    // let x = 0.2857;
    // let epsilon = 0.001;
    // let (num, den) = util::float_to_fraction(x, epsilon);
    // println!("Numerator: {}, Denominator: {}", num, den);
    // println!("{:?}",util::euler1(1000));
    let (result, elapsed) = measure_time(|| {
        println!("{:?}",euler::euler21(10000));
    });
    println!("your_function() executed in: {:?}", elapsed);
    
}


    

    

 