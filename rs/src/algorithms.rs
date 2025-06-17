use crate::algorithms;

pub fn merge_sort(list: &mut Vec<i32>) -> Vec<i32> {
    // 2 1 0 4 5 9 4 3 5 6 9 0 0 -2 1 2 3 5 5 5 6
    // 2 4 5 1 0 8 7

    if list.len() == 1 {
        return list.clone();
    }

    let middle_idx = list.len() / 2;

    let mut end_half = &mut list.split_off(middle_idx);

    let sorted_start_half = merge_sort(list);
    let sorted_end_half = merge_sort(end_half);

    merge_two_sorted_list(&sorted_start_half, &sorted_end_half)
}

// time m+n
// space m+n
pub fn merge_two_sorted_list(list1: &Vec<i32>, list2: &Vec<i32>) -> Vec<i32> {
    let mut result_vec: Vec<i32> = Vec::with_capacity(list1.len() + list2.len());

    let (mut i, mut j) = (0, 0);

    while list1.get(i).is_some() || list2.get(j).is_some() {
        let l1_val = list1.get(i);
        let l2_val = list2.get(j);

        match (l1_val, l2_val) {
            (None, None) => break,
            (None, Some(l2v)) => {
                result_vec.push(*l2v);
                j += 1;
            }
            (Some(l1v), None) => {
                result_vec.push(*l1v);
                i += 1;
            }
            (Some(l1v), Some(l2v)) => {
                if *l1v <= *l2v {
                    result_vec.push(*l1v);
                    i += 1;
                } else {
                    result_vec.push(*l2v);
                    j += 1;
                }
            }
        }
    }
    result_vec
}

pub fn merge_two_sorted_list_rec(list1: &Vec<i32>, list2: &Vec<i32>) -> Vec<i32> {
    // iterative versiyonunu yazdin
    // simdi bu fonksiyonu daha kucuk bir dataset ile calistirmaya calis (iterative versiyonu kullanarak)

    match (list1.first(), list2.first()) {
        (None, None) => Vec::with_capacity(0),
        (None, Some(val)) | (Some(val), None) => Vec::from([*val]),
        (Some(l1_val), Some(l2_val)) => {
            let mut result_vec = Vec::<i32>::new();

            if *l1_val < *l2_val {
                let sorted_rest = algorithms::merge_two_sorted_list_rec(
                    &list1.split_first().unwrap().1.to_vec(),
                    list2,
                );

                result_vec.push(*l1_val);
                result_vec.extend_from_slice(&sorted_rest);
            } else {
                let sorted_rest = algorithms::merge_two_sorted_list_rec(
                    list1,
                    &list2.split_first().unwrap().1.to_vec(),
                );

                result_vec.push(*l2_val);
                result_vec.extend_from_slice(&sorted_rest);
            }

            result_vec
        }
    }
}

// time n
// space n
pub fn sum_of_first_n_natural_rec(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }

    return sum_of_first_n_natural_rec(n - 1) + n;
}

// time 1
// space 1
pub fn sum_of_first_n_natural(n: u128) -> u128 {
    return (n * (n + 1)) / 2;
}

pub fn fac_of_n(n: u128) -> u128 {
    if n == 0 {
        return 1;
    }

    return n * fac_of_n(n - 1);
}
