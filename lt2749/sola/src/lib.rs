pub struct Solution;

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let num1 = num1 as i64;
        let num2 = num2 as i64;
        let mut num = num1;
        let mut count = 0;
        for _ in 0..64 {
            num -= num2;
            count += 1;
            if num.count_ones() <= count && count as i64 <= num {
                return count.try_into().unwrap();
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let res = Solution::make_the_integer_zero(3, 0);
        assert_eq!(res, 2);
    }

    #[test]
    fn test1() {
        let res = Solution::make_the_integer_zero(3, -2);
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let res = Solution::make_the_integer_zero(5, 7);
        assert_eq!(res, -1);
    }

    #[test]
    fn test3() {
        let res = Solution::make_the_integer_zero(112577768, -501662198);
        assert_eq!(res, 16);
    }
}
