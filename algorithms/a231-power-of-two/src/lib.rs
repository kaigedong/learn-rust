struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_power_of_two(n: i32) -> bool {
        n.is_positive() && n.count_ones() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert!(Solution::is_power_of_two(1));
        assert!(Solution::is_power_of_two(2));
        assert!(Solution::is_power_of_two(4));

        assert!(!Solution::is_power_of_two(0));
        assert!(!Solution::is_power_of_two(3));
    }
}
