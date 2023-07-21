pub struct Solution;

impl Solution {
    pub fn find_value_of_partition(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut gap = i32::MAX;
        for i in 0..nums.len() - 1 {
            gap = std::cmp::min(gap, (nums[i] - nums[i + 1]).abs());
        }
        gap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::find_value_of_partition(vec![2, 1, 3, 4]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test2() {
        let res = Solution::find_value_of_partition(vec![10, 100, 1]);
        assert_eq!(res, 9);
    }
}
