use std::collections::HashMap;

fn main() {
    println!("{}", is_valid(String::from("()[]))")));
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
