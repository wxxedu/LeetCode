pub struct Solution;

impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let bytes = s.into_bytes();
        let n = bytes.len();
        let mut res = 0;
        for i in 0..n {
            let mut has_found = false;
            let mut has_break = false;
            for j in i..n {
                if j > i && bytes[j] == bytes[j - 1] {
                    if has_found {
                        res = std::cmp::max(res, (j - i) as i32);
                        has_break = true;
                        break;
                    } else {
                        has_found = true;
                    }
                }
            }
            if !has_break {
                res = std::cmp::max(res, (n - i) as i32)
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res =
            Solution::longest_semi_repetitive_substring("52233".to_owned());
        assert_eq!(res, 4);
    }

    #[test]
    fn test2() {
        let res =
            Solution::longest_semi_repetitive_substring("5494".to_owned());
        assert_eq!(res, 4);
    }

    #[test]
    fn test3() {
        let res =
            Solution::longest_semi_repetitive_substring("1111111".to_owned());
        assert_eq!(res, 2);
    }
}
