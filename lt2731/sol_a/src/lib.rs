pub struct Solution;

const MOD: i64 = 1000_000_007;

impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        let n = nums.len();
        let mut postions = vec![0i64; n];
        let chars = s.into_bytes();
        for i in 0..n {
            if chars[i] == 82 {
                postions[i] = nums[i] as i64 + d as i64;
            } else {
                postions[i] = nums[i] as i64 - d as i64;
            }
        }
        postions.sort();
        let mut res = 0;
        for i in 1..n {
            res += (((((postions[i] - postions[i - 1]) as i64 % MOD)
                * i as i64)
                % MOD)
                * (n - i) as i64)
                % MOD;
            res = res % MOD;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::sum_distance(vec![-2, 0, 2], "RLL".to_owned(), 3);
        assert_eq!(res, 8);
    }

    #[test]
    fn test2() {
        let res = Solution::sum_distance(vec![1, 0], "RL".to_owned(), 2);
        assert_eq!(res, 5);
    }

    #[test]
    fn test3() {
        let res = Solution::sum_distance(
            vec![-10, -13, 10, 14, 11],
            "LRLLR".to_owned(),
            2,
        );
        assert_eq!(res, 146);
    }
}
