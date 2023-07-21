pub struct Solution;
/// 2 3 3 2 2 4 ->

pub struct UnionFind<const N: usize> {
    content: [i32; N],
    count: usize,
}

impl<const N: usize> UnionFind<N> {
    pub fn new() -> Self {
        Self {
            content: [-1; N],
            count: N,
        }
    }

    pub fn union(&mut self, a: usize, b: usize) {
        if self.content[a] == -1 {
            self.content[a] = b as i32;
            self.count -= 1;
        } else if self.content[b] == -1 {
            self.content[b] = a as i32;
            self.count -= 1;
        } else if self.find(a) != self.find(b) {
            let aroot = self.find(a);
            let broot = self.find(b);
            self.content[aroot] = broot as i32;
            self.count -= 1;
        }
    }

    pub fn find(&mut self, val: usize) -> usize {
        if self.content[val] == -1 {
            return val;
        }
        let res = self.find(self.content[val] as usize);
        self.content[val] = res as i32;
        res
    }
}

impl Solution {
    pub fn minimize_concatenated_length(words: Vec<String>) -> i32 {
        let mut res = 0;
        let mut union = UnionFind::<26>::new();
        let mut start_char_counts = [0; 26];
        let mut end_char_counts = [0; 26];
        for word in &words {
            res += word.len() as i32;
            let start_char =
                (word.chars().nth(0).unwrap() as u32 - 97) as usize;
            let end_char = (word.chars().last().unwrap() as u32 - 97) as usize;
            start_char_counts[start_char] += 1;
            end_char_counts[end_char] += 1;
            union.union(start_char, end_char);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = Solution::minimize_concatenated_length(vec![
            "aa".to_owned(),
            "ab".to_owned(),
            "bc".to_owned(),
        ]);
        assert_eq!(res, 4);
    }

    #[test]
    fn test2() {
        let res = Solution::minimize_concatenated_length(vec![
            "ab".to_owned(),
            "b".to_owned(),
        ]);
        assert_eq!(res, 2);
    }

    #[test]
    fn test3() {
        let res = Solution::minimize_concatenated_length(vec![
            "aaa".to_owned(),
            "c".to_owned(),
            "aba".to_owned(),
        ]);
        assert_eq!(res, 6);
    }
}
