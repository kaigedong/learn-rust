pub fn add_binary(a: String, b: String) -> String {
    let mut bin_a = a.chars().rev();
    let mut bin_b = b.chars().rev();

    let mut out: Vec<char> = vec![];
    let mut carry = false;

    let mut out_char;
    loop {
        let a_char = bin_a.next();
        let b_char = bin_b.next();
        match (a_char, b_char) {
            (None, None) => break,
            (Some(c), None) | (None, Some(c)) => {
                (out_char, carry) = add(c, '0', carry);
            }
            (Some(c1), Some(c2)) => {
                (out_char, carry) = add(c1, c2, carry);
            }
        }
        out.push(out_char);
    }
    if carry {
        out.push('1');
    }

    out.into_iter().rev().collect()
}

fn add(a: char, b: char, carry: bool) -> (char, bool) {
    if carry {
        match (a, b) {
            ('1', '1') => ('1', true),  // 1 + 1 + 1
            ('0', '0') => ('1', false), // 1
            _ => ('0', true),           // 1 + 0 + 1
        }
    } else {
        match (a, b) {
            ('1', '1') => ('0', true),
            ('0', '0') => ('0', false),
            _ => ('1', false),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::add;
    use crate::add_binary;
    #[test]
    fn test_add() {
        assert_eq!(add('1', '1', true), ('1', true));
        assert_eq!(add('1', '1', false), ('0', true));
        assert_eq!(add('1', '0', true), ('0', true));
        assert_eq!(add('1', '0', false), ('1', false));
        assert_eq!(add('0', '1', true), ('0', true));
    }

    #[test]
    fn test_add_binary() {
        assert_eq!(add_binary("0".into(), "0".into()), "0");
        assert_eq!(add_binary("1".into(), "1".into()), "10");
        assert_eq!(add_binary("1".into(), "0".into()), "1");
        assert_eq!(add_binary("11".into(), "1".into()), "100");
        assert_eq!(add_binary("1010".into(), "1011".into()), "10101");
        assert_eq!(add_binary("10".into(), "1".into()), "11");
        assert_eq!(add_binary("1111".into(), "111".into()), "10110");
    }
}
