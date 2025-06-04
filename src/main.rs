use std::collections::HashMap;

fn main() {
    let nums_vec = Vec::from([3, 2, 4]);
    let result = two_sum(nums_vec, 6);

    println!("{:?}", result);
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

    for i in 0..nums.len() { // n
        if nums_map.contains_key(&(target - nums[i])) { // 1
            res_vec[0] = i as i32; // 1
            res_vec[1] = *nums_map.get(&(target - nums[i])).unwrap(); // 1

            return res_vec; // 1
        }

        nums_map.insert(nums[i], i as i32); // 1
    }

    res_vec
}
