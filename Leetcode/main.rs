use libc::c_char;
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
use std::{collections::HashMap, vec};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

extern "C" {
    fn hello_world() -> *const c_char;
}

fn call_hello_world() -> &'static str {
    unsafe {
        CStr::from_ptr(hello_world())
            .to_str()
            .expect("String conversion failure")
    }
}

fn main() {
    // let point = Point { x: 1, y: 2 };

    // let serialized = serde_json::to_string(&point).unwrap();
    // println!("serialized = {}", serialized);

    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    // println!("deserialized = {:?}", deserialized);

    println!("{}", call_hello_world());

    // println!("{}", my_atoi(String::from(" ")));
}

pub fn my_atoi(s: String) -> i32 {
    let mut result = 0;
    let mut i = 0;
    let mut sign = 1;

    if s.len() == 0 {
        return 0;
    }

    while i < s.len() && s.chars().nth(i) == Some(' ') {
        i += 1;
    }

    if s.chars().nth(i) == Some('-') {
        sign = -1;
        i += 1
    } else if s.chars().nth(i) == Some('+') {
        sign = 1;
        i += 1
    }

    while i < s.len() && s.chars().nth(i).unwrap() >= '0' && s.chars().nth(i).unwrap() <= '9' {
        let digit = s
            .chars()
            .nth(i)
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap()
            * sign;

        if sign == 1
            && (result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > i32::MAX % 10))
        {
            return i32::MAX;
        }

        if sign == -1
            && (result < i32::MIN / 10 || (result == i32::MIN / 10 && digit < i32::MIN % 10))
        {
            return i32::MIN;
        }

        result = result * 10 + digit;
        i += 1;
    }

    result
}

pub fn reverse(x: i32) -> i32 {
    let mut str = x.to_string();
    let mut is_minus = false;
    let mut rev_str: Vec<String> = vec![];

    if str.contains("-") {
        is_minus = true;
        str = String::from(&str[1..]);
    }

    for x in str.chars().rev() {
        rev_str.push(x.to_string());
    }

    let res_abs = rev_str.join("").parse::<i32>();
    return match res_abs {
        Ok(x) => {
            if is_minus {
                x * -1
            } else {
                x
            }
        }
        Err(_x) => 0,
    };
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits_rev = digits.into_iter().rev().collect::<Vec<_>>();
    let mut i = 0;
    let mut needs_extra = false;

    while let Some(_x) = digits_rev.get(i) {
        if digits_rev[i] == 9 {
            digits_rev[i] = 0;
            if i == digits_rev.len() - 1 {
                needs_extra = true;
            }
        } else {
            digits_rev[i] += 1;
            break;
        }
        i += 1;
    }
    if needs_extra {
        digits_rev.push(1);
    }

    digits_rev.into_iter().rev().collect::<Vec<_>>()
}
// let d_plus_1 = digits
//     .iter()
//     .map(|&x| x.to_string())
//     .collect::<Vec<_>>()
//     .join("")
//     .parse::<i128>()
//     .unwrap()
//     + 1;
// let d_plus_1_vec = d_plus_1
//     .to_string()
//     .chars()
//     .map(|x| x.to_string().parse::<i32>().unwrap())
//     .collect::<Vec<_>>();

pub fn my_sqrt(x: i32) -> i32 {
    // (x as f64).sqrt().round() as i32
    if x == 0 {
        return 0;
    }
    let mut left = 1;
    let mut right = x;
    let mut mid = x / 2;

    while left <= right {
        if mid == 0 {
            break;
        }
        if x / mid < mid {
            right = mid - 1;
        } else if x / mid > mid {
            if x / (mid + 1) < mid + 1 {
                return mid;
            } else {
                left = mid + 1;
            }
        } else {
            return mid;
        }

        mid = left + (right - left) / 2;
    }

    1
}

pub fn is_valid(s: String) -> bool {
    let mut my_vec: Vec<char> = Vec::new();
    let bracket_pairs: HashMap<char, char> = [(')', '('), ('}', '{'), (']', '[')]
        .iter()
        .cloned()
        .collect();

    for c in s.chars() {
        match c {
            '(' | '{' | '[' => my_vec.push(c),
            z @ ')' | z @ '}' | z @ ']' => match my_vec.pop() {
                Some(p) => {
                    if &p != bracket_pairs.get(&z).unwrap() {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            },
            _ => (),
        }
    }
    if my_vec.len() > 0 {
        return false;
    }
    true
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut vec_str: Vec<String> = vec![];
    let mut min_len = std::usize::MAX;

    for s in strs.iter() {
        if s.len() < min_len {
            min_len = s.len();
        }
    }

    for i in 0..min_len {
        let s = strs[0].chars().nth(i).unwrap().to_string();
        let mut is_in_all = true;

        for (j, str) in strs.iter().enumerate() {
            // println!("{}", str.chars().nth(i).unwrap().to_string());
            if j == 0 {
                continue;
            }
            if s != str.chars().nth(i).unwrap().to_string() {
                is_in_all = false;
                break;
            }
        }

        if is_in_all {
            vec_str.push(s);
        } else {
            break;
        }
    }

    vec_str.join("")
}

pub fn is_palindrome(x: i32) -> bool {
    let str = x.to_string();
    let mut rev_str: Vec<String> = vec![];
    for x in str.chars().rev() {
        rev_str.push(x.to_string());
    }
    if rev_str.join("") == str {
        true
    } else {
        false
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let roman = HashMap::from([
        (String::from("I"), 1),
        (String::from("V"), 5),
        (String::from("X"), 10),
        (String::from("L"), 50),
        (String::from("C"), 100),
        (String::from("D"), 500),
        (String::from("M"), 1000),
    ]);

    for (i, x) in s.chars().enumerate() {
        if i == s.len() - 1 {
            break;
        }
        let cur = roman[&x.to_string()];
        let next = roman[&s.chars().nth(i + 1).unwrap().to_string()];
        if cur >= next {
            result += cur;
        } else {
            result -= cur;
        }
    }

    println!(
        "{}",
        result + roman[&s.chars().nth(s.len() - 1).unwrap().to_string()]
    );
    result + roman[&s.chars().nth(s.len() - 1).unwrap().to_string()]
}
