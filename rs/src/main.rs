use core::num;
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    result,
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

// TODO: turkce konus -> error ve overflow durumlarini kontrol et
impl Solution {
    // time
    // space
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => Solution::fib(n - 1) + Solution::fib(n - 2),
        }
    }
    pub fn fib_iterative(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let (mut n1, mut n2) = (0, 1);

                let mut sum = 0;
                let mut i = 2;

                while i <= n {
                    sum = n1 + n2;

                    n1 = n2;
                    n2 = sum;
                    i += 1;
                }

                sum
            }
        }
    }

    // time 2n (n^2) possible
    // space n
    pub fn is_palindrome_iterative(head: Option<Box<ListNode>>) -> bool {
        let mut is_palindrome = true;

        if head.is_none() {
            return false;
        }

        let mut numlist = Vec::<i32>::new();
        let mut current = head;

        loop {
            match current.take() {
                Some(node) => {
                    numlist.push(node.val);
                    current = node.next;
                }
                None => break,
            }
        }

        let (mut i, mut j) = (0, numlist.len() - 1);

        while i < j {
            if numlist[i] != numlist[j] {
                is_palindrome = false;
                break;
            }

            i += 1;
            j -= 1;
        }

        is_palindrome
    }

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        match head {
            Some(node) => true,
            None => false,
        }
    }

    pub fn is_power_of_two(n: i32) -> bool {
        if n < 1 {
            return false;
        } else if n == 1 {
            return true;
        }
        Solution::is_power_of_two(n / 2) && n % 2 == 0
    }

    pub fn is_power_of_two_iterative(n: i32) -> bool {
        let mut copy_n = n;
        let mut result = true;

        if copy_n <= 0 {
            return false;
        }

        while copy_n > 2 {
            let quotient = copy_n / 2;
            if !(quotient % 2 == 0 && copy_n % 2 == 0) {
                result = false;
                break;
            }

            copy_n /= 2;
        }

        result
    }
    pub fn reverse_list_iterative(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut current_node_ptr = head;
        let mut tail_ptr: Option<Box<ListNode>> = None;

        loop {
            let mut node = current_node_ptr.take();

            match node {
                Some(mut n) => {
                    let next_node = std::mem::replace(&mut n.next, tail_ptr.take());
                    tail_ptr = Some(n);
                    current_node_ptr = next_node;
                }
                None => break,
            }
        }

        tail_ptr
    }

    // TODO
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::reverse_list_helper(head, None)
    }

    fn reverse_list_helper(
        head: Option<Box<ListNode>>,
        prev: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                Solution::reverse_list_helper(std::mem::replace(&mut node.next, prev), Some(node))
            }
            None => prev,
        }
    }

    pub fn remove_elements_iterative(
        head: Option<Box<ListNode>>,
        val: i32,
    ) -> Option<Box<ListNode>> {
        let mut new_head_dummy = Some(Box::new(ListNode { val: 0, next: None }));
        let mut tail_ptr = new_head_dummy.as_mut();

        let mut current_node_ptr = head.as_ref();

        while let Some(node) = current_node_ptr {
            if node.val != val {
                let new_node = Some(Box::new(ListNode {
                    val: node.val,
                    next: None,
                }));

                tail_ptr.as_mut().unwrap().next = new_node;
                tail_ptr = tail_ptr.unwrap().next.as_mut();
            }
            current_node_ptr = node.next.as_ref();
        }

        new_head_dummy.unwrap().next
    }

    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match (head) {
            Some(node) => {
                if node.val != val {
                    Some(Box::new(ListNode {
                        val: node.val,
                        next: Solution::remove_elements(node.next, val),
                    }))
                } else {
                    Solution::remove_elements(node.next, val)
                }
            }
            None => None,
        }
    }

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(l2_node)) => Some(l2_node),
            (Some(l1_node), None) => Some(l1_node),
            (Some(l1_node), Some(l2_node)) => {
                if l1_node.val < l2_node.val {
                    Some(Box::new(ListNode {
                        val: l1_node.val,
                        next: Solution::merge_two_lists(l1_node.next, Some(l2_node)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l2_node.val,
                        next: Solution::merge_two_lists(Some(l1_node), l2_node.next),
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
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let mut result_list_head = Solution::reverse_list(list1);
    // let result = Solution::is_power_of_two(3);
    println!("{:#?}", result_list_head);

    let x = Some("123431");

    
}
