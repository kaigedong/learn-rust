pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 1;
    }

    let mut left = 1;
    let mut right = x;

    loop {
        let mid = (right + left) / 2;

        if mid == x / mid {
            return mid;
        }
        if mid > x / mid {
            right = mid;
        } else {
            left = mid;
        }

        if right - left == 1 {
            return left;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::my_sqrt;
    #[test]
    fn test_my_sqrt() {
        assert_eq!(my_sqrt(1), 1);
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(8), 2);
        assert_eq!(my_sqrt(9), 3);
        assert_eq!(my_sqrt(10), 3);
        assert_eq!(my_sqrt(2147395599), 46339);
    }
}
