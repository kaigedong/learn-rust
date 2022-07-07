// 编写一个算法来判断一个数 n 是不是快乐数。
//
// 「快乐数」 定义为：
//
//     对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和。
//     然后重复这个过程直到这个数变为 1，也可能是 无限循环 但始终变不到 1。
//     如果这个过程 结果为 1，那么这个数就是快乐数。
//
// 如果 n 是 快乐数 就返回 true ；不是，则返回 false 。

use std::collections::HashSet;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_happy(n: i32) -> bool {
        let mut test_num = n;
        let mut seen = HashSet::new();
        loop {
            test_num = Self::is_happy_helper(test_num);
            if test_num == 1 {
                return true;
            }
            if seen.contains(&test_num) {
                return false;
            }
            seen.insert(test_num);
        }
    }

    fn is_happy_helper(mut n: i32) -> i32 {
        // 该数字将要被求是否是快乐数
        let mut out = 0;
        while n != 0 {
            out += (n % 10) * (n % 10);
            n /= 10;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert!(Solution::is_happy(19));
        assert!(!Solution::is_happy(2));
    }
}
