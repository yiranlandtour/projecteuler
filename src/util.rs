use std::collections::{HashSet, VecDeque};
extern crate rand;
use rand::seq::SliceRandom;
use rand::Rng;
use num::bigint::BigUint;
// extern crate image;

pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    let target = vec![vec![1, 2, 3], vec![4, 5, 0]];
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((board.clone(), vec![board.clone()]));
    visited.insert(board.clone());

    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    while !queue.is_empty() {
        let (current, path) = queue.pop_front().unwrap();
        if current == target {
            return path;
        }

        let (mut x, mut y) = (0, 0);
        'outer: for i in 0..2 {
            for j in 0..3 {
                if current[i][j] == 0 {
                    x = i;
                    y = j;
                    break 'outer;
                }
            }
        }

        for dir in &directions {
            let new_x = (x as i32 + dir.0) as usize;
            let new_y = (y as i32 + dir.1) as usize;
            if new_x < 2 && new_y < 3 {
                let mut new_board = current.clone();
                new_board[x][y] = new_board[new_x][new_y];
                new_board[new_x][new_y] = 0;
                if !visited.contains(&new_board) {
                    visited.insert(new_board.clone());
                    let mut new_path = path.clone();
                    new_path.push(new_board.clone());
                    queue.push_back((new_board, new_path));
                }
            }
        }
    }
    vec![] // 如果没有解决方案
}

pub fn sliding_puzzle_rand(board: Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    let n = board.len();
    let mut target: Vec<Vec<i32>> = Vec::new();
    let mut num = 1;

    for i in 0..n {
        let mut row = vec![];
        for j in 0..n {
            if i == n - 1 && j == n - 1 {
                row.push(0);
            } else {
                row.push(num);
                num += 1;
            }
        }
        target.push(row);
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((board.clone(), vec![board.clone()]));
    visited.insert(board.clone());

    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    while !queue.is_empty() {
        let (current, path) = queue.pop_front().unwrap();
        if current == target {
            return path;
        }

        let (mut x, mut y) = (0, 0);
        'outer: for i in 0..n {
            for j in 0..n {
                if current[i][j] == 0 {
                    x = i;
                    y = j;
                    break 'outer;
                }
            }
        }

        for dir in &directions {
            let new_x = (x as i32 + dir.0) as usize;
            let new_y = (y as i32 + dir.1) as usize;
            if new_x < n && new_y < n {
                let mut new_board = current.clone();
                new_board[x][y] = new_board[new_x][new_y];
                new_board[new_x][new_y] = 0;
                if !visited.contains(&new_board) {
                    visited.insert(new_board.clone());
                    let mut new_path = path.clone();
                    new_path.push(new_board.clone());
                    queue.push_back((new_board, new_path));
                }
            }
        }
    }
    vec![] // 如果没有解决方案
}

// 判断一个数字是否为质数
pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

// 二分查找函数
// 如果找到返回 Some(index)，否则返回 None
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() as usize;

    while left < right {
        let mid = left + (right - left) / 2;
        if &arr[mid] == target {
            return Some(mid);
        } else if &arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

// a +/-* b = c 求满足条件的情况的公式
fn eval(a: i32, b: i32, op: char) -> Option<i32> {
    match op {
        '+' => a.checked_add(b),
        '-' => a.checked_sub(b),
        '*' => a.checked_mul(b),
        '/' => if b != 0 { a.checked_div(b) } else { None },
        '^' => if b == 2 { a.checked_mul(a) } else { None }, // Only supporting square for simplicity
        _ => None,
    }
}

pub fn find_expression(nums: &mut Vec<i32>, target: i32, expr: String) -> Option<String> {
    if nums.is_empty() {
        return None;
    }
    if nums.len() == 1 {
        if nums[0] == target {
            return Some(expr + &nums[0].to_string());
        } else {
            return None;
        }
    }

    let ops = vec!['+', '-', '*', '/', '^'];
    for (idx, &num) in nums.iter().enumerate() {
        let mut new_nums = nums.clone();
        new_nums.remove(idx);
        for &op in &ops {
            let new_expr = format!("{} {} {}", expr, op, num);
            let new_target = match op {
                '+' => target - num,
                '-' => target + num,
                '*' => if target % num == 0 { target / num } else { continue },
                '/' => if num != 0 && target % num == 0 { target / num } else { continue },
                '^' => if num * num == target { num } else { continue },
                _ => continue,
            };
            if let Some(res) = find_expression(&mut new_nums, new_target, new_expr) {
                return Some(res);
            }
        }
    }
    None
}

//
use std::collections::HashMap;

fn is_valid_expression(expr: &str, mapping: &HashMap<char, i64>) -> bool {
    let mut stack: Vec<i64> = Vec::new();
    let mut num = 0i64;
    let mut sign = 1i64;
    let mut result = 0i64;

    for ch in expr.chars() {
        if let Some(&val) = mapping.get(&ch) {
            num = num * 10 + val;
        } else {
            match ch {
                '+' => {
                    result += sign * num;
                    num = 0;
                    sign = 1;
                }
                '-' => {
                    result += sign * num;
                    num = 0;
                    sign = -1;
                }
                '=' => {
                    result += sign * num;
                    num = 0;
                    sign = -1;  // Reset the sign for the RHS of the equation
                }
                _ => {}
            }
        }
    }
    result += sign * num;
    result == 0
}

fn backtrack(expr: &str, index: usize, used: &mut Vec<bool>, mapping: &mut HashMap<char, i64>) -> bool {
    if index == expr.len() {
        return is_valid_expression(expr, mapping);
    }

    let ch = expr.chars().nth(index).unwrap();
    if mapping.contains_key(&ch) {
        return backtrack(expr, index + 1, used, mapping);
    }

    for i in 0..10 {
        if !used[i as usize] && !(i == 0 && index > 0 && expr.chars().nth(index - 1).is_some() && expr.chars().nth(index - 1).unwrap().is_alphabetic()) {
            used[i as usize] = true;
            mapping.insert(ch, i as i64);
            if backtrack(expr, index + 1, used, mapping) {
                return true;
            }
            used[i as usize] = false;
            mapping.remove(&ch);
        }
    }
    false
}

pub fn find_valid_mapping(expr: &str) -> Option<HashMap<char, i64>> {
    let mut mapping = HashMap::new();
    let mut used = vec![false; 10];

    if backtrack(expr, 0, &mut used, &mut mapping) {
        return Some(mapping);
    }
    None
}

//随机格子
// use rand::Rng;

const N: usize = 30;
const M: usize = 30;

fn print_grid(grid: &[[char; M]; N]) {
    for row in grid.iter() {
        for &ch in row.iter() {
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

pub fn simulate_expansion() {
    let mut rng = rand::thread_rng();
    let (mut x, mut y) = (rng.gen_range(0..N), rng.gen_range(0..M));

    let mut grid = [['.'; M]; N];
    grid[x][y] = '*';

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    print_grid(&grid);

    loop {
        let mut new_grid = grid.clone();
        for i in 0..N {
            for j in 0..M {
                if grid[i][j] == '*' {
                    for &(dx, dy) in directions.iter() {
                        let new_x = i as isize + dx;
                        let new_y = j as isize + dy;

                        if new_x >= 0 && new_x < N as isize && new_y >= 0 && new_y < M as isize {
                            new_grid[new_x as usize][new_y as usize] = '*';
                        }
                    }
                }
            }
        }

        if new_grid == grid {
            break;
        }

        grid = new_grid;
        print_grid(&grid);
    }
}

pub fn simulate_interaction2() {
    let mut rng = rand::thread_rng();
    let (mut x_star, mut y_star) = (rng.gen_range(0..N), rng.gen_range(0..M));
    let (mut x_plus, mut y_plus);

    loop {
        x_plus = rng.gen_range(0..N);
        y_plus = rng.gen_range(0..M);
        if x_star != x_plus || y_star != y_plus {
            break;
        }
    }

    let mut grid = [['.'; M]; N];
    grid[x_star][y_star] = '*';
    grid[x_plus][y_plus] = '+';

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    print_grid(&grid);

    loop {
        let mut new_grid = grid.clone();
        for i in 0..N {
            for j in 0..M {
                if grid[i][j] == '*' {
                    for &(dx, dy) in directions.iter() {
                        let new_x = i as isize + dx;
                        let new_y = j as isize + dy;

                        if new_x >= 0 && new_x < N as isize && new_y >= 0 && new_y < M as isize {
                            match grid[new_x as usize][new_y as usize] {
                                '.' => new_grid[new_x as usize][new_y as usize] = '*',
                                '+' => new_grid[new_x as usize][new_y as usize] = '.',
                                _ => {},
                            }
                        }
                    }
                } else if grid[i][j] == '+' {
                    for &(dx, dy) in directions.iter() {
                        let new_x = i as isize + dx;
                        let new_y = j as isize + dy;

                        if new_x >= 0 && new_x < N as isize && new_y >= 0 && new_y < M as isize {
                            match grid[new_x as usize][new_y as usize] {
                                '.' => new_grid[new_x as usize][new_y as usize] = '+',
                                '*' => new_grid[new_x as usize][new_y as usize] = '.',
                                _ => {},
                            }
                        }
                    }
                }
            }
        }

        // if new_grid == grid {
        //     break;
        // }
        let dot_count = new_grid.iter().flatten().filter(|&&x| x == '.').count();
        if dot_count <= (N * M) / 10 || new_grid == grid {
            break;
        }

        grid = new_grid;
        print_grid(&grid);
    }
}



// use image::{GrayImage, ImageBuffer, imageops, Luma};
// use std::path::Path;

// pub fn image_to_ascii<P: AsRef<Path>>(path: P) -> Vec<Vec<char>> {
//     // 加载图片并转换为灰度
//     let img = image::open(path).unwrap().to_luma();

//     // 缩放图片
//     let resized = imageops::resize(&img, 1024, 1024, imageops::FilterType::Triangle);

//     // 将灰度值映射到 ASCII 字符
//     let ascii_chars = ["@", "#", "8", "&", "o", ":", "*", ".", " "];
//     let ascii_img: Vec<Vec<char>> = resized.enumerate_pixels().map(|(_, _, pixel)| {
//         let gray = pixel[0] as usize;
//         let char_idx = gray * (ascii_chars.len() - 1) / 255;
//         ascii_chars[char_idx].into()
//     }).collect::<Vec<_>>();

//     // 将一维向量转换为二维
//     ascii_img.chunks(1024).map(|chunk| chunk.to_vec()).collect()
// }

//use 回溯法将所有的组合都找出来
fn backtrack0(nums: &[i32], target: i32, start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if target == 0 && !current.is_empty() {

        result.push(current.clone());
        println!("{:?}",result);
        return;
    }
    if start == nums.len() {
        return;
    }

    for i in start..nums.len() {
        if i > start && nums[i] == nums[i - 1] {
            continue;
        }
        if nums[i] > target {
            break;
        }
        current.push(nums[i]);
        backtrack0(nums, target - nums[i], i + 1, current, result);
        current.pop();
    }
}

pub fn sum_combinations(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let mut result = Vec::new();
    let mut current = Vec::new();
    backtrack0(&nums, target, 0, &mut current, &mut result);
    result
}

// fn mid_search(nums: Vec<i32>,target: i32) ->i32{
//     // nums = nums.borrow();
//     nums.sort_unstable();
//     let mut i = 0;
//     let mut j = nums.len();
//     let mut mid = (i + j)/2;

//     while i < j{
//         if target == nums[mid]{
//             return mid as i32;
//         }
//         if target < nums[mid]{
//             j = mid;
//         }else{
//             i = mid + 1;
//         }
//         mid = (i + j)/2;
//     }

//     return mid as i32;
// }

pub fn rand_go(){
    let step = 2000;
    let mut rng = rand::thread_rng();
    let mut all_num = 0;
    let mut target = 0;
    for i in 0..100000{
        let mut sanzi = 0;
        while sanzi < step+1{
            sanzi += rng.gen_range(1..7);
            // println!("{:?}",sanzi);
            if sanzi == step{
                target += 1;
            }
            
        }
        all_num += 1;
    }
    println!("{:?},{:?}",target,all_num);
}

pub fn float_to_fraction(x: f64, epsilon: f64) -> (i64, i64) {
    let mut z = x;
    let mut a0 = z.floor() as i64;
    let mut a = a0;
    println!("{:?},{:?}",a0,a);
    let mut num1: i64 = 1;
    let mut num2: i64 = a;
    let mut den1: i64 = 0;
    let mut den2: i64 = 1;

    while (num2 as f64 / den2 as f64 - x).abs() > epsilon {
        z = 1.0 / (z - a as f64);
        a = z.floor() as i64;
        println!("{:?},{:?}",z,a);
        let num = num1 + a * num2;
        let den = den1 + a * den2;
        
        num1 = num2;
        den1 = den2;
        num2 = num;
        den2 = den;
        println!("{:?},{:?},{:?},{:?}",num1,num2,den1,den2);
    }

    (num2, den2)
}
// oular
pub fn euler1(x:i32)-> i32{
    let mut res = 0;
    for i in 3..x{
        if i % 3 == 0 || i % 5 == 0{
            println!("{:?}",i);
            res += i;
        }
    }
    res
}




