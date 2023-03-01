use std::collections::HashMap;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return 0.0;
        }
        let mut seen_pow = HashMap::new();
        seen_pow.insert(0, 1.0);
        seen_pow.insert(1, x);

        if n < 0 {
            1.0 / Self::my_pow_helper(n.abs(), &mut seen_pow)
        } else {
            Self::my_pow_helper(n, &mut seen_pow)
        }
    }

    // 当n % 2 == 0:
    // x^n = x^(n/2) * x^(n/2)
    // 当n % 2 == 1:
    // x^n = x^(n/2) * x^(n/2) * x
    fn my_pow_helper(n: i32, seen_pow: &mut HashMap<i32, f64>) -> f64 {
        // 存储已经在HashMap中存储的x的多少次方的值
        if seen_pow.contains_key(&n) {
            *seen_pow.get(&n).unwrap()
        } else {
            let out = if n % 2 == 0 {
                Self::my_pow_helper(n / 2, seen_pow) * Self::my_pow_helper(n / 2, seen_pow)
            } else {
                Self::my_pow_helper(n / 2, seen_pow)
                    * Self::my_pow_helper(n / 2, seen_pow)
                    * Self::my_pow_helper(1, seen_pow)
            };
            *seen_pow.entry(n).or_insert(out)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_my_pow() {
        assert_eq!(Solution::my_pow(1.0, 1), 1.0);
        assert_eq!(Solution::my_pow(0.0, 2), 0.0);
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);

        assert_eq!(Solution::my_pow(0.00001, 2147483647), 0.0);
        assert_eq!(Solution::my_pow(1.0, 2147483647), 1.0);
    }
}
