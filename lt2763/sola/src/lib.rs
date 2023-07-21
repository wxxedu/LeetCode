use std::backtrace;

pub struct Solution;

#[derive(Clone, PartialEq, Eq)]
pub struct Mask {
    start: i32,
    end: i32,
    is_decreasing: bool,
    has_inbetween: bool,
}

impl Mask {
    #[inline]
    pub fn new(a: i32, b: i32) -> Self {
        let start;
        let end;
        let is_decreasing;
        if a > b {
            start = b;
            end = a;
            is_decreasing = true;
        } else {
            start = a;
            end = b;
            is_decreasing = false;
        }
        Self {
            start,
            end,
            is_decreasing,
            has_inbetween: false,
        }
    }

    #[inline]
    pub fn check(&self) -> bool {
        self.has_inbetween
    }

    #[inline]
    pub fn mark(&mut self, num: i32) {
        if num >= self.end || num <= self.start {
            return;
        }
        self.has_inbetween = true;
    }
}

impl Solution {
    #[inline]
    pub fn sum_imbalance_numbers(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() - 1 {
            let this = nums[i];
            let that = nums[i + 1];
            if (this - that).abs() > 1 {
                let mut mask = Mask::new(this, that);
                let mut mask2 = mask.clone();
                res += 1;
                let mut backward_count = 0;
                for j in (i + 2)..nums.len() {
                    let other = nums[j];
                    mask.mark(other);
                    if mask.check() {
                        break;
                    }
                    backward_count += 1;
                }

                let mut forward_count = 0;

                for j in 0..i {
                    let j = i - 1 - j;
                    let other = nums[j];
                    mask2.mark(other);
                    if mask2.check() {
                        break;
                    }
                    if other == that {
                        break;
                    }
                    forward_count += 1;
                }

                res += forward_count
                    + backward_count
                    + forward_count * backward_count;
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
        let res = Solution::sum_imbalance_numbers(vec![2, 3, 1, 4]);
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let res = Solution::sum_imbalance_numbers(vec![1, 3, 3, 3, 5]);
        assert_eq!(res, 8);
    }

    #[test]
    fn test3() {
        let res = Solution::sum_imbalance_numbers(vec![1, 3, 1]);
        assert_eq!(res, 3);
    }

    #[test]
    fn test4() {
        let res = Solution::sum_imbalance_numbers(vec![1, 1, 3, 1]);
        assert_eq!(res, 5);
    }

    #[test]
    fn test5() {
        let res = Solution::sum_imbalance_numbers(vec![1, 1, 3, 3]);
        assert_eq!(res, 4);
    }

    #[test]
    fn test6() {
        let res = Solution::sum_imbalance_numbers(vec![1, 1, 1, 3, 3]);
        assert_eq!(res, 6);
    }
}
