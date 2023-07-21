pub struct Solution;

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]);
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let res = Solution::paint_walls(vec![2, 3, 4, 2], vec![1, 1, 1, 1]);
        assert_eq!(res, 4);
    }
}
