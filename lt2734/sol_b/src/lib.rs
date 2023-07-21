pub struct Solution;

impl Solution {
    pub fn smallest_string(s: String) -> String {
        let mut bytes = s.into_bytes();
        let length = bytes.len();
        let mut has_started = false;
        for i in 0..length {
            if bytes[i] == 97 {
                if has_started {
                    break;
                } else {
                    continue;
                }
            }
            has_started = true;
            bytes[i] -= 1;
        }
        if !has_started {
            bytes[length - 1] = 'z' as u8;
        }
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::smallest_string("cbabc".to_owned());
        assert_eq!(res, "baabc".to_owned());
    }

    #[test]
    fn test2() {
        let res = Solution::smallest_string("acbbc".to_owned());
        assert_eq!(res, "abaab".to_owned());
    }

    #[test]
    fn test3() {
        let res = Solution::smallest_string("leetcode".to_owned());
        assert_eq!(res, "kddsbncd".to_owned());
    }

    #[test]
    fn test4() {
        let res = Solution::smallest_string("a".to_owned());
        assert_eq!(res, "z".to_owned());
    }
}
