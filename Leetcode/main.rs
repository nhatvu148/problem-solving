mod components;
use components::comp_1::*;

fn main() {
    let mut obj = StockSpanner::new();
    println!("{}", obj.next(100));
    println!("{}", obj.next(80));
    println!("{}", obj.next(60));
    println!("{}", obj.next(70));
    println!("{}", obj.next(60));
    println!("{}", obj.next(75));
    println!("{}", obj.next(85));
}
