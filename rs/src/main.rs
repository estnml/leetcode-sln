use core::num;
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

mod algorithms;

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

// TODO: add proper error handling & overflow controls
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // sliding window problemi gibi.

        let mut longest_w_start = 0;
        let mut longest_w_end = 0;

        let mut current_w_start = 0;
        let mut current_w_end = 0; // ?

        for (idx, ch) in s.char_indices() {}

        todo!()
    }

    pub fn merge_two_lists_rec(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(l2_node)) => Some(Box::new(ListNode {
                val: l2_node.val,
                next: Solution::merge_two_lists_rec(l2_node.next, None),
            })),
            (Some(l1_node), None) => Some(Box::new(ListNode {
                val: l1_node.val,
                next: Solution::merge_two_lists_rec(l1_node.next, None),
            })),
            (Some(l1_node), Some(l2_node)) => {
                if l1_node.val < l2_node.val {
                    Some(Box::new(ListNode {
                        val: l1_node.val,
                        next: Solution::merge_two_lists_rec(l1_node.next, Some(l2_node)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l2_node.val,
                        next: Solution::merge_two_lists_rec(Some(l1_node), l2_node.next),
                    }))
                }
            }
        }
    }

    pub fn merge_two_lists_iterative(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut new_list_head: Option<Box<ListNode>> =
            Some(Box::new(ListNode { val: 0, next: None }));
        let mut tail_ptr = new_list_head.as_mut();

        let mut current_l1_node = list1.as_ref();
        let mut current_l2_node = list2.as_ref();

        while current_l1_node.is_some() || current_l2_node.is_some() {
            match (current_l1_node, current_l2_node) {
                (None, None) => break,
                (None, Some(node)) => {
                    tail_ptr.as_mut().unwrap().next = Some(Box::new(ListNode {
                        val: node.val,
                        next: None,
                    }));
                    current_l2_node = node.next.as_ref();
                }
                (Some(node), None) => {
                    tail_ptr.as_mut().unwrap().next = Some(Box::new(ListNode {
                        val: node.val,
                        next: None,
                    }));
                    current_l1_node = node.next.as_ref();
                }
                (Some(l1_node), Some(l2_node)) => {
                    if l1_node.val < l2_node.val {
                        let new_node = Some(Box::new(ListNode {
                            val: l1_node.val,
                            next: None,
                        }));

                        tail_ptr.as_mut().unwrap().next = new_node;
                        current_l1_node = l1_node.next.as_ref();
                    } else {
                        let new_node = Some(Box::new(ListNode {
                            val: l2_node.val,
                            next: None,
                        }));
                        tail_ptr.as_mut().unwrap().next = new_node;
                        current_l2_node = l2_node.next.as_ref();
                    }
                }
            }

            tail_ptr = tail_ptr.unwrap().next.as_mut();
        }

        new_list_head.unwrap().next
    }

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let reversed = Solution::reverse_list(head.unwrap().next);
        println!("reversed: {:?}", reversed.unwrap().val);

        None
    }

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

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None, // base condition
            (None, Some(ln)) | (Some(ln), None) => Some(ln),
            (Some(l1), Some(l2)) => {
                let sum = l1.val + l2.val;

                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(l1.next, l2.next),
                    }))
                } else {
                    let carry_node = Some(Box::new(ListNode {
                        val: sum / 10,
                        next: None,
                    }));

                    Some(Box::new(ListNode {
                        val: sum % 10,
                        next: Solution::add_two_numbers(
                            Solution::add_two_numbers(l1.next, carry_node),
                            l2.next,
                        ),
                    }))
                }
            }
        }
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
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let mut result_list_head = Solution::merge_two_lists(list1, list2);
    println!("{:#?}", result_list_head);
    // Solution::reverse_list(list1);
}
