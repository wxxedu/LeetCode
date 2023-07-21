pub struct Solution;

const DIVISOR: i64 = 1000_000_007;

impl Solution {
    pub fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {
        let mut res: i64 = 1;
        let mut nums_since_last_one = 0;
        let mut has_found_one = false;
        for val in nums.iter() {
            nums_since_last_one += 1;
            if *val == 1 {
                if has_found_one {
                    res = (res * nums_since_last_one) % DIVISOR;
                }
                nums_since_last_one = 0;
                has_found_one = true;
            }
        }
        if has_found_one {
            res as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::number_of_good_subarray_splits(vec![1]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test2() {
        let res = Solution::number_of_good_subarray_splits(vec![0, 1, 0, 0, 1]);
        assert_eq!(res, 3);
    }

    #[test]
    fn test3() {
        let res = Solution::number_of_good_subarray_splits(vec![0, 1, 0]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test4() {
        let res = Solution::number_of_good_subarray_splits(vec![0, 0]);
        assert_eq!(res, 0);
    }

    #[test]
    fn test5() {
        let res = Solution::number_of_good_subarray_splits(vec![
            0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1,
            0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1,
            1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1,
            0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1,
            0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0,
            1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1,
            0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0,
            1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1,
            1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0,
            0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0,
            1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1,
            0,
        ]);
        assert_eq!(res, 230630552);
    }
}
