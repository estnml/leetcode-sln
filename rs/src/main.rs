use std::collections::HashMap;

mod structure_impls;

fn main() {
    // println!("{}", "\"".len());
    // let mut word = Vec::from(['H', 'a', 'n', 'n', 'a', 'h']);
    reverse_string(&mut Vec::from(['\"']));
    // println!("{:?}", word);
}

// time n
// space n
fn length_of_last_word<T: AsRef<str>>(value: T) -> i32 {
    let mut current_word_len = 0;
    let mut len_vec = Vec::<i32>::new();
    for char in value.as_ref().chars() {
        if char == ' ' {
            if current_word_len != 0 {
                len_vec.push(current_word_len);
                current_word_len = 0;
            }

            continue;
        }

        current_word_len += 1;
    }

    if current_word_len > 0 {
        len_vec.push(current_word_len);
    }

    println!("{:?}", len_vec);
    *len_vec.last().unwrap()
}

// time n
// space n
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_map = std::collections::HashMap::<i32, i32>::new(); // 1
    let mut res_vec: Vec<i32> = Vec::from([0, 0]); // 1

    // n
    for i in 0..nums.len() {
        if nums_map.contains_key(&(target - nums[i])) {
            // 1
            res_vec[0] = i as i32; // 1
            res_vec[1] = *nums_map.get(&(target - nums[i])).unwrap(); // 1

            return res_vec; // 1
        }

        nums_map.insert(nums[i], i as i32); // 1
    }

    res_vec
}

// Definition for singly-linked list.
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
}
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    todo!()
}

// h e l l o
pub fn reverse_string(s: &mut Vec<char>) {
    let (mut i, mut j) = (0 as usize, s.len() - 1);

    if s.len() == 1 {
        return;
    }

    while i < j {
        s.swap(i, j);
        i += 1;
        j -= 1;
    }

    println!("{:?}", s);
}

pub fn reverse_string_rec(s: &mut Vec<char>) {
    if s.len() <= 1 {
        // 1
        return;
    }

    let arr_len = s.len() - 1; // 1
    s.swap(0, arr_len); // 1?
    reverse_string_rec(&mut s[1..s.len()].to_vec());
}
