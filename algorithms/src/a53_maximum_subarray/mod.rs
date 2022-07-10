pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut pointer = 1;
    let mut max_seen = nums[0];
    let mut max_current = nums[0];

    loop {
        if pointer >= nums.len() {
            break;
        }

        if max_current <= 0 || max_current + nums[pointer] < 0 {
            max_current = nums[pointer];
        } else {
            max_current += nums[pointer];
        }

        if max_current > max_seen {
            max_seen = max_current;
        }

        pointer += 1;
    }
    max_seen
}

#[cfg(test)]
mod tests {
    use super::max_sub_array;
    #[test]
    fn test1() {
        let one_vec = vec![1];
        assert_eq!(max_sub_array(one_vec), 1);

        let one_vec = vec![-1];
        assert_eq!(max_sub_array(one_vec), -1);

        let one_vec = vec![1, -1];
        assert_eq!(max_sub_array(one_vec), 1);

        let one_vec = vec![2, -5];
        assert_eq!(max_sub_array(one_vec), 2);

        let one_vec = vec![-2, -1];
        assert_eq!(max_sub_array(one_vec), -1);

        let one_vec = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array(one_vec), 6);

        let one_vec = vec![5, 4, -1, 7, 8];
        assert_eq!(max_sub_array(one_vec), 23);
    }
}
