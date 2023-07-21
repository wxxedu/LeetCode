pub struct Solution;

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        if nums.len() <= 1 {
            return nums.len() as i64;
        }
        // invariant: the interval [start, end] will always be valid.
        let mut start = 0;
        let mut end = 0;
        let mut max = nums[0];
        let mut min = nums[0];
        let mut res: i64 = 0;
        while end < nums.len() - 1 {
            // look out for here ^
            let next_val = nums[end + 1];

            if next_val <= min + 2 && next_val >= max - 2 {
                let vals = [next_val, min, max];
                min = *vals.iter().min().unwrap();
                max = *vals.iter().max().unwrap();
                end += 1;
                continue;
            } else {
                let diff = end - start + 1;
                res += ((diff + 1) * diff / 2) as i64;
                let mut curr_min = next_val;
                let mut curr_max = next_val;
                let mut last_min = curr_min;
                let mut last_max = curr_max;
                for i in 0..end + 2 {
                    let curr_idx = end + 1 - i;
                    let curr_val = nums[curr_idx];
                    let vals = [curr_val, curr_min, curr_max];
                    curr_min = *vals.iter().min().unwrap();
                    curr_max = *vals.iter().max().unwrap();

                    if curr_max - curr_min > 2 {
                        start = curr_idx + 1;
                        break;
                    }
                    last_min = curr_min;
                    last_max = curr_max;
                }
                min = last_min;
                max = last_max;
                let diff = end - start + 1;
                res -= ((diff + 1) * diff / 2) as i64;
                end += 1;
            }
        }

        let diff = end - start + 1;
        res += ((diff + 1) * diff / 2) as i64;
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::continuous_subarrays(vec![1, 2, 3, 4, 5]);
        assert_eq!(res, 12);
    }

    #[test]
    fn test2() {
        let res = Solution::continuous_subarrays(vec![1, 1, 1, 1, 1]);
        assert_eq!(res, 15);
    }
}

fn main() {}
