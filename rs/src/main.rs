use core::num;
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

struct Solution;

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

impl Solution {
    // time n
    // space n?
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut window_start = 0;
        let mut max_window_size = 0;
        let mut current_window_size = 0;
        let mut window_map = std::collections::HashMap::<char, usize>::new();

        for (idx, ch) in s.char_indices() {
            if !window_map.contains_key(&ch) {
                window_map.insert(ch, idx);
                current_window_size += 1;
            } else {
                if !window_start >= *window_map.get(&ch).unwrap() {
                    window_start = *window_map.get(&ch).unwrap() + 1;
                }

                window_map.insert(ch, idx);
                current_window_size = idx - window_start + 1;
            }

            if current_window_size > max_window_size {
                max_window_size = current_window_size;
            }
        }

        max_window_size as i32
    }

    // time max(n,m)
    // space max(n,m)
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        let mut current_l1_node = l1.as_ref();
        let mut current_l2_node = l2.as_ref();

        let mut carry = 0;

        while current_l1_node.is_some() || current_l2_node.is_some() {
            let l1_val = if let Some(l1_node) = current_l1_node {
                current_l1_node = l1_node.next.as_ref();
                l1_node.val
            } else {
                0
            };

            let l2_val = if let Some(l2_node) = current_l2_node {
                current_l2_node = l2_node.next.as_ref();
                l2_node.val
            } else {
                0
            };

            let current_sum = l1_val + l2_val + carry;
            carry = current_sum / 10;

            tail.next = Some(Box::new(ListNode::new(current_sum % 10)));
            tail = tail.next.as_mut().unwrap();
        }

        if carry > 0 {
            tail.next = Some(Box::new(ListNode::new(carry)));
        }

        dummy.next
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

    pub fn is_palindrome(x: i32) -> bool {
        let mut xcopy = x;
        let mut reverse = 0;
        if x < 0 {
            return false;
        }
        while xcopy > 0 {
            reverse = (reverse * 10) + (xcopy % 10);
            xcopy /= 10;
        }
        x == reverse
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
            return;
        }

        let arr_len = s.len() - 1; // 1
        s.swap(0, arr_len); // 1?
        Solution::reverse_string_rec(&mut s[1..s.len()].to_vec());
    }
}

fn main() {
    let res = Solution::length_of_longest_substring("pwwkew".to_string());
    println!("{}", res);
}
