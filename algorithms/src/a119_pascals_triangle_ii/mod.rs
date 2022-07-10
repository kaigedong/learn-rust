use std::collections::HashMap;

struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut out = vec![];
        let mut vals = HashMap::new();

        for i in 0..=row_index {
            out.push(Self::get_val(row_index, i, &mut vals));
        }
        out
    }

    fn get_val(row_index: i32, col_index: i32, vals: &mut HashMap<(i32, i32), i32>) -> i32 {
        if row_index == 0 || col_index == 0 || col_index == row_index {
            return 1;
        }

        // TODO: 等待这个特性稳定
        // let left_before = vals.try_insert(
        //     (row_index - 1, col_index - 1),
        //     Self::get_val(row_index - 1, col_index - 1, vals),
        // );

        let left_before: i32 = if vals.contains_key(&(row_index - 1, col_index - 1)) {
            *vals.get(&(row_index - 1, col_index - 1)).unwrap()
        } else {
            let left_before = Self::get_val(row_index - 1, col_index - 1, vals);

            vals.insert((row_index - 1, col_index - 1), left_before);
            left_before
        };

        let right_before: i32 = if vals.contains_key(&(row_index - 1, col_index)) {
            *vals.get(&(row_index - 1, col_index)).unwrap()
        } else {
            let right_before = Self::get_val(row_index - 1, col_index, vals);
            vals.insert((row_index - 1, col_index), right_before);
            right_before
        };

        left_before + right_before
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_row() {
        assert_eq!(Solution::get_row(2), vec![1, 2, 1]);
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }
}
