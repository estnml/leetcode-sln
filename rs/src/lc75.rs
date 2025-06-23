struct Solution;

impl Solution {
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
        todo!()
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
        let mut result_gcd = String::with_capacity(std::cmp::max(str1.len(), str2.len()));

        result_gcd
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

    let result = Solution::can_place_flowers(flowerbed, new_flowers);
    println!("{:?}", result);
}
