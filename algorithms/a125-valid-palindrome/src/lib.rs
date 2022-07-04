struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(s: String) -> bool {
        let s_chars: Vec<_> = s.to_ascii_lowercase().chars().collect();

        let char_num = s_chars.len();
        if char_num <= 1 {
            return true;
        }

        let mut left = 0;
        let mut right = char_num - 1;

        loop {
            if right <= left {
                return true;
            }

            if !s_chars[left].is_ascii_alphanumeric() {
                left += 1;
                continue;
            }
            if !s_chars[right].is_ascii_alphanumeric() {
                right -= 1;
                continue;
            }

            if s_chars[left] != s_chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_valid_palindrome() {
        let a1 = "A man, a plan, a canal: Panama".to_string();
        let a2 = "race a car".to_string();

        assert!(Solution::is_palindrome(a1));
        assert!(!Solution::is_palindrome(a2));
    }
}
