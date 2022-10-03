pub fn longest_palindrome_sub_string(s: String) -> String {
    let mut longest_palindrome = (0, 0);
    let s_chars: Vec<char> = s.chars().collect();
    for i in 0..s.len() as i32 {
        let odd_palindrome = palindrome_sub_string(&s_chars, i, i);
        let even_palindrome = palindrome_sub_string(&s_chars, i, i + 1);
        let tmp_palindrome =
            if odd_palindrome.1 - odd_palindrome.0 > even_palindrome.1 - even_palindrome.0 {
                odd_palindrome
            } else {
                even_palindrome
            };
        longest_palindrome =
            if tmp_palindrome.1 - tmp_palindrome.0 > longest_palindrome.1 - longest_palindrome.0 {
                tmp_palindrome
            } else {
                longest_palindrome
            }
    }
    return s[longest_palindrome.0 as usize..longest_palindrome.1 as usize].to_string();
}

fn palindrome_sub_string(s: &Vec<char>, start: i32, end: i32) -> (i32, i32) {
    let length = s.len() as i32;
    let mut left = start;
    let mut right = end;
    while left >= 0 && right < length && s[left as usize] == s[right as usize] {
        left -= 1;
        right += 1;
    }
    (left + 1, right)
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome_sub_string;

    #[test]
    fn test_longest_palidrome_sub_string() {
        let lps = longest_palindrome_sub_string("ababab".to_string());
        println!("{}", lps);
    }
}
