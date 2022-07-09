struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // can choose 0 or 1 as start point
        let mut has_cost = vec![0, 0];
        for i in 2..(cost.len() + 1) {
            // when in position i, it will cost at least:
            let next_min_cost =
                i32::min(has_cost[i - 1] + cost[i - 1], has_cost[i - 2] + cost[i - 2]);
            has_cost.push(next_min_cost);
        }
        has_cost[has_cost.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
