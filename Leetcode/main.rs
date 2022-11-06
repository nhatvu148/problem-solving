mod components;
use std::num::ParseIntError;

use components::comp_1::*;

#[derive(Debug)]
struct SummationError;

fn to_int(s: &str) -> Result<i32, ParseIntError> {
    // Option<i32> {
    s.parse()
    // s.parse().ok()
}

fn sum_str_vec(strs: Vec<String>) -> Result<String, SummationError> {
    // Option<String> {
    let mut accum = 0i32;
    for s in strs {
        accum += to_int(&s).map_err(|_| SummationError)?;
        // accum += to_int(&s).ok_or(SummationError)?;
        // accum += to_int(&s)?;
    }

    Ok(accum.to_string())
    // Some(accum.to_string())
}

fn main() {
    let v = vec![String::from("3"), String::from("4")];
    let total = sum_str_vec(v);
    println!("{:?}", total);

    let v = vec![String::from("3"), String::from("abc")];
    let total = sum_str_vec(v);
    println!("{:?}", total);
}
