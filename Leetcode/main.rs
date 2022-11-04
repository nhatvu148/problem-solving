use libc::c_char;
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
use std::vec;
mod components;
use components::comp_1::*;

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

    // println!("{}", call_hello_world());

    // let my_number = 1;
    // println!("my_number memory location {:p}", &my_number);
    // let my_number = 1;
    // let my_number_pointer: *const i32 = &my_number;
    // println!("my_number memory location {:p}", my_number_pointer);

    // let mut my_number = 1;
    // println!("my_number memory location {:p}", &mut my_number);
    // // or
    // let mut my_number = 1;
    // // let my_number_pointer: *const i32 = &mut my_number;
    // let my_number_pointer: *mut i32 = &mut my_number;
    // println!("my_number memory location {:p}", my_number_pointer);

    let l1 = vec![2, 4, 3];
    let l2 = vec![5, 6, 4];

    let linked_list1 = create_linked_list_from_vec(l1);
    println!("{:?}", &linked_list1);

    let linked_list2 = create_linked_list_from_vec(l2);
    println!("{:?}", &linked_list1);

    println!(
        "{:?}",
        add_two_numbers(Some(linked_list1), Some(linked_list2))
    );
}
