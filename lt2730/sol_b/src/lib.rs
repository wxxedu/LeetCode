pub struct Solution;

impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let bytes = s.into_bytes();
        let n = bytes.len();
        let mut res = 1;
        let mut i = 0;
        let mut j = 1;
        let mut last = 0;
        while j < n {
            if bytes[j] == bytes[j - 1] {
                if last > 0 {
                    i = last;
                }
                last = j;
            }
            res = std::cmp::max(res, (j - i + 1) as i32);
            j += 1;
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
