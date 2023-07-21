pub struct Solution;

impl Solution {
    pub fn longest_string(x: i32, y: i32, z: i32) -> i32 {
        let val = std::cmp::min(x, y);
        let mut res = 2 * val + z;
        if x != y {
            res += 1;
        }
        res * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::longest_string(2, 5, 1);
        assert_eq!(res, 12);
    }

    #[test]
    fn test2() {
        let res = Solution::longest_string(3, 2, 2);
        assert_eq!(res, 14);
    }
}
