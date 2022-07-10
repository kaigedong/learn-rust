struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut daily_earned = Vec::with_capacity(prices.len());

        daily_earned.push(0);
        for i in 1..prices.len() {
            daily_earned.push(prices[i] - prices[i - 1]);
        }

        Self::max_profit_helper(&daily_earned)
    }

    // 贪心算法
    fn max_profit_helper(earned: &[i32]) -> i32 {
        let mut max_seen = 0;
        let mut max_current = 0;

        for one_earned in earned {
            if max_current + one_earned >= 0 {
                max_current += one_earned;
                max_seen = i32::max(max_current, max_seen);
            } else {
                max_current = 0;
            }
        }
        max_seen
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_max_profile() {
        let a = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(a), 5);
        let a = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(a), 0);
    }
}
