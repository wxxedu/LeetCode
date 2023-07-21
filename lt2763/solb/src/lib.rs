pub struct Solution;

impl Solution {
    #[inline]
    fn count_imbalance_for_pair(
        start: usize,
        end: usize,
        nums: &Vec<i32>,
    ) -> i32 {
        let start_val = nums[start];
        let end_val = nums[end];

        let (lower, higher) = if start_val > end_val {
            (end_val, start_val)
        } else {
            (start_val, end_val)
        };

        let mut forward_count = 0;
        for i in (0..start).rev() {
            let other = nums[i];
            if other > lower && other < higher {
                break;
            }
            if other == end_val {
                break;
            }
            forward_count += 1;
        }

        let mut backward_count = 0;
        for i in end + 1..nums.len() {
            let other = nums[i];
            if other > lower && other < higher {
                break;
            }
            backward_count += 1;
        }

        forward_count + backward_count + forward_count * backward_count + 1
    }

    #[inline]
    pub fn sum_imbalance_numbers(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() - 1 {
            let this = nums[i];
            let mut last_max: Option<i32> = None;
            let mut no_more_max = false;
            let mut last_min: Option<i32> = None;
            let mut no_more_min = false;
            for j in i + 1..nums.len() {
                if no_more_min && no_more_max {
                    break;
                }
                let next = nums[j];
                if next == this {
                    break;
                } else if next > this {
                    if next < this + 2 {
                        no_more_max = true;
                        continue;
                    }
                    if no_more_max {
                        continue;
                    }
                    match last_max {
                        Some(val) => {
                            if next < val {
                                last_max = Some(next);
                                res +=
                                    Self::count_imbalance_for_pair(i, j, &nums);
                            }
                        }
                        None => {
                            last_max = Some(next);
                            res += Self::count_imbalance_for_pair(i, j, &nums);
                        }
                    }
                } else {
                    if next > this - 2 {
                        no_more_min = true;
                        continue;
                    }
                    if no_more_min {
                        continue;
                    }
                    match last_min {
                        Some(val) => {
                            if next > val {
                                last_min = Some(next);
                                res +=
                                    Self::count_imbalance_for_pair(i, j, &nums);
                            }
                        }
                        None => {
                            last_min = Some(next);
                            res += Self::count_imbalance_for_pair(i, j, &nums);
                        }
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test1() {
        let res = Solution::sum_imbalance_numbers(vec![2, 3, 1, 4]);
        assert_eq!(res, 3);
    }

    #[ignore]
    #[test]
    fn test2() {
        let res = Solution::sum_imbalance_numbers(vec![1, 3, 3, 3, 5]);
        assert_eq!(res, 8);
    }

    #[ignore]
    #[test]
    fn test3() {
        let res = Solution::sum_imbalance_numbers(vec![1, 3, 1]);
        assert_eq!(res, 3);
    }

    #[ignore]
    #[test]
    fn test4() {
        let res = Solution::sum_imbalance_numbers(vec![1, 1, 3, 1]);
        assert_eq!(res, 5);
    }

    #[ignore]
    #[test]
    fn test5() {
        let res = Solution::sum_imbalance_numbers(vec![1, 1, 3, 3]);
        assert_eq!(res, 4);
    }

    #[ignore]
    #[test]
    fn test6() {
        let res = Solution::sum_imbalance_numbers(vec![1, 1, 1, 3, 3]);
        assert_eq!(res, 6);
    }

    #[ignore]
    #[test]
    fn test7() {
        let res = Solution::sum_imbalance_numbers(vec![1, 2, 3]);
        assert_eq!(res, 0);
    }

    #[test]
    fn test8() {
        let res = Solution::sum_imbalance_numbers(vec![3, 1, 1]);
        assert_eq!(res, 2);
    }
}
