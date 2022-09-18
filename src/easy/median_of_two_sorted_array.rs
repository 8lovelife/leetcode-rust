pub fn median_of_two_sorted_array(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let nums1_len = nums1.len();
    let nums2_len = nums2.len();
    let mut nums3 = Vec::with_capacity(nums1_len + nums2_len);
    while i < nums1_len && j < nums2_len {
        if nums1[i] < nums2[j] {
            nums3.insert(k, nums1[i]);
            i += 1;
        } else {
            nums3.insert(k, nums2[j]);
            j += 1;
        }
        k += 1;
    }
    if i == nums1_len {
        for jk in j..nums2_len {
            nums3.insert(k, nums2[jk]);
            k += 1;
        }
    } else {
        for ik in i..nums1_len {
            nums3.insert(k, nums1[ik]);
            k += 1;
        }
    }
    let median_index = nums3.len() / 2;
    if nums3.len() % 2 == 0 {
        (nums3[median_index] + nums3[median_index - 1]) as f64 / 2.0
    } else {
        nums3[median_index] as f64
    }
}

#[cfg(test)]
mod tests {
    use super::median_of_two_sorted_array;

    #[test]
    fn test_median_of_two_sorted_array() {
        let nums1 = [1, 3];
        let nums2 = [2, 4];
        let median = median_of_two_sorted_array(nums1.to_vec(), nums2.to_vec());
        println!("median value is {}", median);
    }
}
