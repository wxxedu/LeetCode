pub struct Solution;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
        let n = nums.len();
        let mut prefixes = vec![vec![0; n]; n];
        let mut res = 0;
        for i in 0..n {
            prefixes[0][i] = nums[i];
            res += nums[i] as i64;
        }
        for i in 1..n {
            let mut line_sum = (i as i64) * (x as i64);
            for j in 0..n {
                prefixes[i][j] =
                    std::cmp::min(prefixes[i - 1][j], prefixes[0][(i + j) % n]);
                line_sum += prefixes[i][j] as i64;
            }
            if line_sum < res {
                res = line_sum;
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
        let res = Solution::min_cost(vec![20, 1, 15], 5);
        assert_eq!(res, 13);
    }

    #[test]
    fn test2() {
        let res = Solution::min_cost(vec![1, 2, 3], 4);
        assert_eq!(res, 6);
    }
}
