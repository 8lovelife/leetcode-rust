use std::{cmp, collections::HashMap};

pub fn length_longest_sub_string(s: String) -> i32 {
    let mut pre_char_map = HashMap::with_capacity(s.len());
    let mut longest_length = 0;
    let mut start = 0;
    for (i, v) in s.char_indices() {
        if let Some(pre_index) = pre_char_map.get(&v){
            start = cmp::max(pre_index + 1, start);
        }
        pre_char_map.insert(v, i);
        longest_length = cmp::max(longest_length, i - start + 1);
    }
    longest_length as i32
}

#[cfg(test)]
mod tests {

    use super::length_longest_sub_string;
    #[test]
    fn test_length_of_longest_sub_string() {
        let length = length_longest_sub_string("dvdq".to_string());
        println!("length of longest sub string is {}", length);
    }
}
