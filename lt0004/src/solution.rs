struct Solution;

/// Goal: to find the median of the given 2 vectors.
impl Solution {
    fn find_median(nums: Vec<i32>) -> Result<f64, ()> {
        if nums.len() == 0 {
            return Err(());
        }
        let mid = nums.len() / 2;
        if nums.len() % 2 == 0 {
            Ok(f64::from(nums[mid] + nums[mid - 1]) / 2.0)
        } else {
            Ok(f64::from(nums[mid]))
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() == 0 {
            return Solution::find_median(nums2).unwrap();
        }
        if nums2.len() == 0 {
            return Solution::find_median(nums1).unwrap();
        }
        let length = nums1.len() + nums2.len();
        let mut mid1 = nums1.len() / 2;
        let mut mid2 = nums2.len() / 2;
        if length % 2 == 0 {

        } else {

        }
        todo!()
    }
}

#[cfg(test)]
mod test {
    use float_cmp::approx_eq;
    use crate::solution::Solution;

    #[test]
    fn should_find_median_of_one_array_when_there_are_odd_number_of_elements() {
        let arr = vec![1, 2, 3];
        let res = Solution::find_median(arr);
        assert!(res.is_ok());
        assert!(approx_eq!(f64, res.unwrap(), 2.0, ulps=2))
    }

    #[test]
    fn should_find_median_of_one_array_when_there_are_even_number_of_elements() {
        let arr = vec![1, 2, 3, 4];
        let res = Solution::find_median(arr);
        assert!(res.is_ok());
        assert!(approx_eq!(f64, res.unwrap(), 2.5, ulps=2));
    }
}