#![allow(dead_code, unused_imports)]
mod components;
use components::comp::*;
use components::comp_1::*;

fn main() {
    println!("{}", remove_duplicates(String::from("azxxzy")));
}
