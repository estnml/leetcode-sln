fn main() {
    let nums_vec = Vec::from([3,3]);
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


// time n^2
// space
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() { // n
        if nums[i] > target {
            continue;
        }

        let current_index = i;
        let current_num = nums[i];
        for j in current_index + 1..nums.len() { // n-1 -> n*(n-1) -> n^2
            if target - current_num == nums[j] {
                return Vec::from([current_index as i32, j as i32]);
            }
        }
    }

    Vec::from([0, 0])
}
