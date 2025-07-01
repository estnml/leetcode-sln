struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }

    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        todo!()
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut current_area = 0;
        let mut result_max_area = 0;

        let mut start_ptr = 0;
        let mut end_ptr = height.len() - 1;

        while start_ptr < end_ptr {
            current_area =
                (end_ptr - start_ptr) as i32 * std::cmp::min(height[end_ptr], height[start_ptr]);

            if current_area > result_max_area {
                result_max_area = current_area;
            }

            if height[start_ptr] < height[end_ptr] {
                start_ptr += 1;
            } else {
                end_ptr -= 1;
            }
        }

        result_max_area
    }
    pub fn reverse_words(s: String) -> String {
        // "the sky is blue" -> "blue is sky the"
        // "  hello world  " -> "world hello"
        // "a good   example" -> "example good a"

        todo!()
    }
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut i = 0;
        let mut current_working_ch: Option<char> = None;
        let mut current_working_ch_count: i32 = 0;

        let mut push_index = 0;

        loop {
            if i == chars.len() {
                break;
            }

            if current_working_ch.is_some_and(|cwc| cwc == chars[i]) {
                current_working_ch_count += 1;
            } else if current_working_ch.is_none() {
                current_working_ch = Some(chars[i]);
                current_working_ch_count += 1;
            } else {
                // karakter degisimi
                chars[push_index] = current_working_ch.unwrap();
                push_index += 1;
                if current_working_ch_count > 1 {
                    if current_working_ch_count <= 9 {
                        chars[push_index] =
                            char::from_digit(current_working_ch_count as u32, 10).unwrap();
                        push_index += 1;
                    } else {
                        // birden fazla basamagi olan bir sayi:
                        let mut x = 1;

                        while current_working_ch_count / x >= 10 {
                            x *= 10;
                        }

                        while x > 0 {
                            chars[push_index] =
                                char::from_digit((current_working_ch_count / x) as u32, 10)
                                    .unwrap();
                            push_index += 1;
                            current_working_ch_count %= 10;
                            x /= 10;
                        }
                    }
                }

                current_working_ch = Some(chars[i]);
                current_working_ch_count = 1;
            }

            i += 1;
        }

        if current_working_ch_count > 0 {
            chars[push_index] = current_working_ch.unwrap();
            push_index += 1;
            if current_working_ch_count > 1 {
                if current_working_ch_count <= 9 {
                    chars[push_index] =
                        char::from_digit(current_working_ch_count as u32, 10).unwrap();
                    push_index += 1;
                } else {
                    // birden fazla basamagi olan bir sayi:
                    let mut x = 1;

                    while current_working_ch_count / x >= 10 {
                        x *= 10;
                    }

                    while x > 0 {
                        chars[push_index] =
                            char::from_digit((current_working_ch_count / x) as u32, 10).unwrap();
                        push_index += 1;
                        current_working_ch_count %= 10;
                        x /= 10;
                    }
                }
            }
        }

        chars[..push_index].len() as i32
    }
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut currently_searching_for: Option<char> = None;
        let mut subseq_iter = s.chars();
        currently_searching_for = subseq_iter.next();

        for ch in t.chars() {
            if currently_searching_for.is_some_and(|search_ch| search_ch == ch) {
                currently_searching_for = subseq_iter.next();
            }
        }

        currently_searching_for.is_none()
    }

    // space complexity azaltmaya calis
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut result = true; // t1
        let mut occurence_map = std::collections::HashMap::<i32, usize>::new(); // t1sn
        let mut distinct_map = std::collections::HashSet::<usize>::new(); // t1sn

        for num in arr {
            // tn
            if occurence_map.contains_key(&num) {
                // t1
                occurence_map
                    .entry(num)
                    .and_modify(|occ_count| *occ_count += 1); // t1
            } else {
                occurence_map.insert(num, 1); // t1
            }
        }

        for occ_value in occurence_map.values() {
            // tn
            if !distinct_map.insert(*occ_value) {
                // t1
                result = false; // t1
                break; // t1
            }
        }

        result
    }
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        // [1,12,-5,-6,50,3] / k = 4
        let (mut w_start, mut w_end): (usize, usize) = (0, 0);
        let (mut max_avg, mut current_max_avg, mut w_sum): (f64, f64, f64) = (0_f64, 0_f64, 0_f64);

        while w_end < nums.len() {
            if w_end - w_start == (k) as usize {
                w_sum = w_sum - nums[w_start] as f64 + nums[w_end] as f64;
                current_max_avg = w_sum / k as f64;
                if current_max_avg > max_avg {
                    max_avg = current_max_avg;
                }

                w_start += 1;
            } else {
                w_sum += nums[w_end] as f64;
                max_avg = w_sum / (w_end + 1) as f64;
            }

            w_end += 1;
        }

        max_avg
    }
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut shift_idx: usize = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, shift_idx);
                shift_idx += 1;
            }
        }
    }

    pub fn reverse_vowels(s: String) -> String {
        let vowels_map = std::collections::HashSet::<char>::from(['a', 'e', 'i', 'o', 'u']);
        let mut char_vec = s.chars().collect::<Vec<char>>();

        let mut start_ptr = 0;
        let mut end_ptr = char_vec.len() - 1;

        while start_ptr < end_ptr {
            if vowels_map.contains(&char_vec[start_ptr].to_ascii_lowercase()) {
                while start_ptr < end_ptr
                    && !vowels_map.contains(&char_vec[end_ptr].to_ascii_lowercase())
                {
                    end_ptr -= 1;
                }

                if start_ptr != end_ptr && start_ptr < end_ptr {
                    char_vec.swap(start_ptr, end_ptr);
                    end_ptr -= 1;
                }
            }

            start_ptr += 1;
        }

        char_vec.into_iter().collect()
    }

    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut num_placable = 0;
        let mut last_non_empty_idx: Option<usize> = None;

        for (idx, elemenet) in flowerbed.iter().enumerate() {
            if *elemenet == 1 {
                last_non_empty_idx = Some(idx);
                continue;
            } else {
                let next = flowerbed.get(idx + 1);
                let prev = flowerbed.get(idx - 1);

                match (prev, next) {
                    (None, None) => {
                        num_placable += 1;
                    }
                    (None, Some(elm)) | (Some(elm), None) => {
                        // su anki eleman listenin basindaki veya sonundaki
                        if idx == flowerbed.len() - 1 {
                            // son eleman kontrolu, sadece onceki deger kontrol edilmeli
                            if !last_non_empty_idx.is_some_and(|lnei| lnei == idx - 1) && *elm == 0
                            {
                                num_placable += 1;
                                last_non_empty_idx = Some(idx);
                            }
                        } else {
                            // ilk eleman, sadece sonraki deger kontrol edilecek.
                            if *elm == 0 {
                                num_placable += 1;
                                last_non_empty_idx = Some(idx);
                            }
                        }
                    }
                    (Some(prev_elm), Some(next_elm)) => {
                        if *prev_elm == 0
                            && *next_elm == 0
                            && last_non_empty_idx.is_some_and(|lnei| idx - 1 != lnei)
                        {
                            num_placable += 1;
                            last_non_empty_idx = Some(idx);
                        }
                    }
                }
            }
        }

        num_placable >= n
    }
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut result_arr: Vec<bool> = Vec::with_capacity(candies.len());

        let candies_iter = candies.iter();
        let max = if let Some(current_max) = candies_iter.max() {
            *current_max
        } else {
            0
        };

        for candy in candies {
            if candy + extra_candies >= max {
                result_arr.push(true);
            } else {
                result_arr.push(false);
            }
        }

        result_arr
    }
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        todo!()
    }
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result_str = String::with_capacity(word1.len() + word2.len());

        let mut w1_iter = word1.chars().peekable();
        let mut w2_iter = word2.chars().peekable();

        loop {
            w1_iter.next().map(|w1_ch| result_str.push(w1_ch));
            w2_iter.next().map(|w2_ch| result_str.push(w2_ch));

            if w1_iter.peek().is_none() && w2_iter.peek().is_none() {
                break;
            }
        }

        result_str
    }
}

fn main() {
    let word1 = String::from("ABABAB");
    let word2 = String::from("ABAB");

    let candies = Vec::from([4, 2, 1, 1, 2]);
    let extra_candies = 1;

    let flowerbed = Vec::from([1, 0, 0, 0, 0, 1]);
    let new_flowers = 2;

    let numlist1 = Vec::from([1, 2, 2, 1, 1, 3]);
    let numlist2 = Vec::from([1, 2, 2, 1, 1, 3]);
    let input_str = String::from("IceCreAm");

    let result = Solution::reverse_vowels(input_str);
    println!("{:?}", result);
}
