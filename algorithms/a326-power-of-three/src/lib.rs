struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_power_of_three(mut n: i32) -> bool {
        if n == 1 {
            return true;
        }
        while n > 0 {
            if n == 3 {
                return true;
            }

            if n % 3 != 0 {
                return false;
            }
            n /= 3;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert!(Solution::is_power_of_three(1));
        assert!(Solution::is_power_of_three(3));
        assert!(Solution::is_power_of_three(9));
        assert!(Solution::is_power_of_three(27));
        assert!(Solution::is_power_of_three(27 * 3));
        assert!(Solution::is_power_of_three(27 * 9));

        assert!(!Solution::is_power_of_three(0));
        assert!(!Solution::is_power_of_three(-1));
        assert!(!Solution::is_power_of_three(-2));
        assert!(!Solution::is_power_of_three(-3));
        assert!(!Solution::is_power_of_three(2));
        assert!(!Solution::is_power_of_three(4));
        assert!(!Solution::is_power_of_three(5));
        assert!(!Solution::is_power_of_three(6));
    }
}
