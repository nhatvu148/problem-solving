#![allow(dead_code, unused_imports)]
use libc::c_char;
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::CStr;
use std::num::ParseIntError;

#[cfg(test)]
#[path = "../unit_tests/comp_1.rs"]
mod comp_1_tests;

pub fn sum_subarray_mins(mut arr: Vec<i32>) -> i32 {
    let mut stk: Vec<usize> = Vec::new();
    let mut ans: i64 = 0;
    const MOD: i64 = 10_i64.pow(9) + 7;
    let min_val: i32 = i32::MIN;
    arr.push(min_val);

    arr.iter().enumerate().for_each(|(i, x)| {
        while !stk.is_empty() && arr[stk[stk.len() - 1]] > *x {
            let top = stk.pop().unwrap();
            let l = if stk.is_empty() {
                top
            } else {
                (top - stk[stk.len() - 1] - 1)
            };
            let r = (i - top - 1);
            let count = ((l + 1) * (r + 1)) as i64;
            ans = (ans + (arr[top] as i64) * (count)) % MOD;
        }
        stk.push(i);
    });
    return ans as i32;
}

pub fn minus(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    vec![a[0] - b[0], a[1] - b[1]]
}
pub fn dot(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    a[0] * b[0] + a[1] * b[1]
}
pub fn distance(a: &Vec<i32>) -> f64 {
    (dot(a, a) as f64).sqrt()
}
pub fn angle(a: &Vec<i32>, pivot: &Vec<i32>, b: &Vec<i32>) -> f64 {
    let bo = minus(&b, &pivot);
    let ao = minus(&a, &pivot);
    dot(&bo, &ao) as f64 / distance(&ao) / distance(&bo)
}
pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut trees = trees.clone();
    if trees.len() <= 3 {
        return trees;
    }
    let mut ret = Vec::new();
    let (mut left_idx, mut left_value, mut y_value) = (0, 101, 101);
    for (i, tree) in trees.iter().enumerate() {
        if tree[0] < left_value || (tree[0] == left_value && tree[1] < y_value) {
            left_idx = i;
            left_value = tree[0];
            y_value = tree[1];
        }
    }

    let mut pivot = &trees[left_idx].clone();
    let mut pivot_idx = left_idx;
    ret.push(pivot.clone());

    let mut reference = &vec![pivot[0], pivot[1] + 101];
    let mut ref_idx: usize = 99999;

    let mut seen = HashSet::new();

    loop {
        let (mut next_idx, mut m_angle, mut m_dis) = (0 as usize, 10.0f64, 1000f64);
        for (i, tree) in trees.iter().enumerate() {
            if i == pivot_idx || i == ref_idx || seen.contains(&i) {
                continue;
            }
            let cur_angle = angle(reference, pivot, tree);
            let cur_dis = distance(&minus(tree, pivot));
            if cur_angle + 0.00001 < m_angle || (cur_angle < m_angle + 0.00001 && cur_dis < m_dis) {
                m_angle = cur_angle;
                m_dis = cur_dis;
                next_idx = i;
            }
        }
        if next_idx == left_idx {
            break;
        }
        seen.insert(next_idx);
        reference = pivot;
        ref_idx = pivot_idx;
        pivot = &trees[next_idx];
        pivot_idx = next_idx;
        ret.push(trees[next_idx].clone());
    }
    ret
}

pub fn is_ugly(n: i32) -> bool {
    let mut n = n;
    if n < 1 {
        return false;
    }
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else if n % 3 == 0 {
            n /= 3;
        } else if n % 5 == 0 {
            n /= 5;
        } else {
            return false;
        }
    }
    true
}

pub fn compute_area(
    ax1: i32,
    ay1: i32,
    ax2: i32,
    ay2: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
) -> i32 {
    let s = max(0, min(ax2, bx2) - max(ax1, bx1)) * max(0, min(ay2, by2) - max(ay1, by1));
    (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1) - s
}

pub fn guess_number(n: i32) -> i32 {
    let (mut low, mut high) = (1, n);
    loop {
        let mid = low + (high - low) / 2;
        match guess(mid) {
            0 => break mid,
            -1 => high = mid - 1,
            _ => low = mid + 1,
        }
    }
}

fn guess(num: i32) -> i32 {
    if num > 1150769282 {
        return -1;
    }

    if num < 1150769282 {
        return 1;
    }

    0
}

pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let len_sts: usize = stones.len();
    let graph = build_graph(&stones);
    let mut seen: HashSet<i32> = HashSet::with_capacity(len_sts);
    let mut cnt: i32 = 0;
    for stone in stones {
        for idx in 0..2 {
            let u = if idx == 0 { stone[0] } else { !stone[1] };
            if seen.insert(u) {
                cnt += 1;
                dfs(u, &mut seen, &graph);
            }
        }
    }
    len_sts as i32 - cnt
}

pub fn dfs(cur: i32, seen: &mut HashSet<i32>, graph: &HashMap<i32, Vec<i32>>) {
    if let Some(nxts) = graph.get(&cur) {
        for &nxt in nxts {
            if seen.insert(nxt) {
                dfs(nxt, seen, graph);
            }
        }
    }
}

pub fn build_graph(stones: &Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let len_sts: usize = stones.len();
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::with_capacity(len_sts);
    for stone in stones {
        let u = stone[0];
        let v = !stone[1];
        graph.entry(u).or_default().push(v);
        graph.entry(v).or_default().push(u);
    }
    graph
}

pub fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

#[derive(Default)]
pub struct MedianFinder {
    data: Vec<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        let mut l = 0;
        let mut r = self.data.len();
        while l < r {
            let mid = l + (r - l) / 2;
            if self.data[mid] < num {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if l < self.data.len() && self.data[l] < num {
            l += 1;
        }
        self.data.insert(l, num);
    }

    fn find_median(&self) -> f64 {
        let len = self.data.len();
        let mut median = 0.0;
        if len % 2 == 0 {
            median = (self.data[len / 2 - 1] + self.data[len / 2]) as f64 / 2.0;
        } else {
            median = self.data[len / 2] as f64;
        }
        median
    }
}

pub fn remove_vec_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 2 {
        return n as i32;
    }
    let mut j = 1;
    for i in 1..n {
        if nums[i] != nums[i - 1] {
            nums[j] = nums[i];
            j += 1;
        }
    }
    println!("{:?}", nums);
    j as i32
}

pub fn remove_duplicates(s: String) -> String {
    let is_dup = |c1: char, c2: char| c1 == c2;
    s.chars()
        .into_iter()
        .fold(Vec::with_capacity(s.len()), |mut v: Vec<char>, c: char| {
            match v.last() {
                Some(prev_c) if is_dup(c, *prev_c) => {
                    v.pop();
                }
                _ => v.push(c),
            }
            v
        })
        .iter()
        .collect()
}

pub struct StockSpanner {
    daily_prices: Vec<i32>,
}

impl StockSpanner {
    pub fn new() -> Self {
        StockSpanner {
            daily_prices: vec![],
        }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        self.daily_prices.push(price);
        let mut counter = 0;
        for p in self.daily_prices.iter().rev() {
            if *p <= price {
                counter += 1;
            } else {
                break;
            }
        }
        counter
    }
}

pub fn make_good(s: String) -> String {
    let diff_cases =
        |c1: char, c2: char| c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2;
    s.chars()
        .into_iter()
        .fold(Vec::with_capacity(s.len()), |mut v: Vec<char>, c| {
            match v.last() {
                Some(prev_c) if diff_cases(c, *prev_c) => {
                    v.pop();
                }
                _ => v.push(c),
            }
            v
        })
        .iter()
        .collect()
}

pub fn maximum69_number(num: i32) -> i32 {
    let mut max = num;
    let num_str = num.to_string();

    for (i, s) in num_str.chars().enumerate() {
        let mut num_str_cpy = num_str.clone();
        if s == '6' {
            num_str_cpy.replace_range(i..i + 1, "9");
            let num_cpy = num_str_cpy.parse::<i32>().unwrap();
            if num_cpy > max {
                max = num_cpy;
            }
        }
    }

    max
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    nums.len() as i32
}

pub fn slide_window_example1() {
    let input = vec![1, 3, 2, 6, -1, 4, 1, 8, 2];
    println!("{:?}", find_averages_of_subarrays(input, 5));
}

pub fn find_averages_of_subarrays(v: Vec<i32>, k: usize) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];
    let mut window_sum = 0.0;
    let mut window_start = 0;

    for (window_end, element) in v.iter().enumerate() {
        window_sum += *element as f32;
        if window_end >= k - 1 {
            result.push(window_sum / k as f32);
            window_sum -= v[window_start] as f32;
            window_start += 1;
        }
    }

    result
}

pub fn result_example() {
    let v = vec![String::from("3"), String::from("4")];
    let total = sum_str_vec(v);
    println!("{:?}", total);

    let v = vec![String::from("3"), String::from("abc")];
    let total = sum_str_vec(v);
    println!("{:?}", total);
}

#[derive(Debug)]
pub struct SummationError;

pub fn to_int(s: &str) -> Result<i32, ParseIntError> {
    // Option<i32> {
    s.parse()
    // s.parse().ok()
}

pub fn sum_str_vec(strs: Vec<String>) -> Result<String, SummationError> {
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

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

extern "C" {
    fn hello_world() -> *const c_char;
}

pub fn call_hello_world() -> &'static str {
    unsafe {
        CStr::from_ptr(hello_world())
            .to_str()
            .expect("String conversion failure")
    }
}

pub fn pointer_example() {
    let my_number = 1;
    println!("my_number memory location {:p}", &my_number);
    let my_number = 1;
    let my_number_pointer: *const i32 = &my_number;
    println!("my_number memory location {:p}", my_number_pointer);

    let mut my_number = 1;
    println!("my_number memory location {:p}", &mut my_number);
    // or
    let mut my_number = 1;
    // let my_number_pointer: *const i32 = &mut my_number;
    let my_number_pointer: *mut i32 = &mut my_number;
    println!("my_number memory location {:p}", my_number_pointer);

    let mut s = String::from("Hello world");
    println!("{}", s);
    let p = s.as_mut_ptr();
    let c = unsafe { *p.add(6) } as char;
    println!("{}", c);
    unsafe { *p.add(6) = 'W' as u8 };
    println!("{}", s);
}

pub fn serialized_example() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

pub fn create_linked_list_from_vec(l: Vec<i32>) -> Box<ListNode> {
    let mut linked_list = Box::new(ListNode::new(l[0]));
    for (i, x) in l.into_iter().enumerate() {
        if i == 0 {
            continue;
        }
        linked_list.insert(x);
    }
    linked_list
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn insert(&mut self, val: i32) {
        let node = Box::new(ListNode::new(val));
        if self.next == None {
            self.next = Some(node);
        } else {
            self.next.as_mut().unwrap().insert(val);
        }
    }

    fn add(&mut self, list_node: ListNode) -> Box<Self> {
        let val = self.val + list_node.val;
        let mut is_over_10 = false;
        let mut node = Box::new(ListNode::new(if val >= 10 {
            is_over_10 = true;
            val - 10
        } else {
            val
        }));
        let mut p1 = self.next.clone();
        let mut p2 = list_node.next.clone();
        let mut next = &mut node;
        while p1 != None || p2 != None || is_over_10 {
            let unwrapped_p1 = match p1 {
                Some(p) => p,
                None => Box::new(ListNode::new(0)),
            };
            let unwrapped_p2 = match p2 {
                Some(p) => p,
                None => Box::new(ListNode::new(0)),
            };
            let val = unwrapped_p1.val + unwrapped_p2.val + if is_over_10 { 1 } else { 0 };
            let next_node = Box::new(ListNode::new(if val >= 10 {
                is_over_10 = true;
                val - 10
            } else {
                is_over_10 = false;
                val
            }));

            next.next = Some(next_node);
            next = next.next.as_mut().unwrap();

            p1 = unwrapped_p1.next;
            p2 = unwrapped_p2.next;
        }
        // println!("{:?}", node);

        node
    }
}

// let l1 = vec![1, 2, 4];
// let l2 = vec![1, 3, 4];

// let linked_list1 = create_linked_list_from_vec(l1);
// println!("{:?}", &linked_list1);

// let linked_list2 = create_linked_list_from_vec(l2);
// println!("{:?}", &linked_list1);

// println!(
//     "{:?}",
//     merge_two_lists(Some(linked_list1), Some(linked_list2))
// );

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    use std::mem;

    let mut dummy = None;
    let mut p_next = &mut dummy;

    while list1.is_some() && list2.is_some() {
        let lone = &mut list1;
        let ltwo = &mut list2;
        let l = if lone.as_ref().unwrap().val < ltwo.as_ref().unwrap().val {
            lone
        } else {
            ltwo
        };

        mem::swap(p_next, l);
        mem::swap(l, &mut p_next.as_mut().unwrap().next);
        p_next = &mut p_next.as_mut().unwrap().next;
    }

    mem::swap(
        p_next,
        if list1.is_none() {
            &mut list2
        } else {
            &mut list1
        },
    );
    dummy
}

// Some(ListNode { val: 2, next: Some(ListNode { val: 4, next: Some(ListNode { val: 3, next: None }) }) })
// Some(ListNode { val: 5, next: Some(ListNode { val: 6, next: Some(ListNode { val: 4, next: None }) }) })
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    Some(l1?.add(*l2.unwrap()))
}

fn is_vowel(ch: char) -> bool {
    let lowered_ch = ch.to_lowercase().to_string().chars().next();
    if lowered_ch == Some('a')
        || lowered_ch == Some('i')
        || lowered_ch == Some('u')
        || lowered_ch == Some('e')
        || lowered_ch == Some('o')
    {
        true
    } else {
        false
    }
}

pub fn reverse_vowels2(s: String) -> String {
    let mut low = 0;
    let mut high = s.len() - 1;
    let mut s = s;

    while low < high {
        let is_low_vowel = is_vowel(s.chars().nth(low).unwrap());
        let is_high_vowel = is_vowel(s.chars().nth(high).unwrap());
        if is_low_vowel && is_high_vowel {
            let mut chs = s.chars().collect::<Vec<char>>();
            chs.swap(low, high);
            s = chs
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("");
            low += 1;
            high -= 1;
        } else if !is_low_vowel {
            low += 1;
        } else if !is_high_vowel {
            high -= 1;
        }
    }
    s
}

pub fn reverse_vowels(s: String) -> String {
    const VOWELS: [char; 10] = ['a', 'i', 'u', 'e', 'o', 'A', 'I', 'U', 'E', 'O'];
    let mut vowel_vec: Vec<char> = vec![];
    let mut result: Vec<char> = vec![];
    for x in s.chars() {
        if VOWELS.contains(&x) {
            vowel_vec.push(x);
        }
    }

    let rev_vowel_vec: Vec<char> = vowel_vec.into_iter().rev().collect();
    let mut key = 0;
    for x in s.chars() {
        if VOWELS.contains(&x) {
            result.push(rev_vowel_vec[key]);
            key += 1;
        } else {
            result.push(x);
        }
    }

    result
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![];
    for (i1, n1) in nums.iter().enumerate() {
        match nums.iter().position(|&r| r == target - n1) {
            Some(i2) if i2 != i1 => {
                result.push(i1 as i32);
                result.push(i2 as i32);
                break;
            }
            _ => {}
        }
    }

    result
}

pub fn int_to_roman(num: i32) -> String {
    let normal = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let roman = vec![
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];
    let mut result: Vec<&str> = vec![];
    let mut num = num;
    for i in 0..13 {
        while num >= normal[i] {
            result.push(roman[i]);
            num -= normal[i];
        }
    }
    result
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>()
        .join("")
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
