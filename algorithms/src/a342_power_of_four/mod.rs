struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_power_of_four(n: i32) -> bool {
        n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert!(Solution::is_power_of_four(1));
        assert!(!Solution::is_power_of_four(3));
        assert!(Solution::is_power_of_four(4));
        assert!(!Solution::is_power_of_four(5));
        assert!(Solution::is_power_of_four(16));
    }
}
