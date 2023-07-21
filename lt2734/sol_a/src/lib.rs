pub struct Solution;

impl Solution {
    pub fn smallest_string(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        let mut should_decrease = true;
        let mut chars = s.chars().peekable();
        loop {
            if let Some(first) = chars.peek() {
                if *first as u8 == 97 {
                    let first = chars.next();
                    res.push(first.unwrap());
                    continue;
                }
            }
            break;
        }
        let mut has_decreased = false;
        for char in &mut chars {
            if (char as u8 - 1) < 97 {
                should_decrease = false;
            }
            if should_decrease {
                res.push((char as u8 - 1) as char);
                has_decreased = true;
            } else {
                res.push(char);
            }
        }
        if !has_decreased {
            let last = res.pop().unwrap();
            res.push(((last as u8 - 97 + 26 - 1) % 26 + 97) as char);
        }
        res
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
