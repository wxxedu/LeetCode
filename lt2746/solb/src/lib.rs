pub struct Solution;

fn print_dp(val: &Vec<[[i32; 26]; 26]>) {
    for i in 0..val.len() {
        println!("Level {}", i + 1);
        for line in val[i] {
            for x in line {
                print!("{} ", x);
            }
            println!();
        }
        println!();
    }
}

pub trait MyTrait {
    fn start(&self) -> usize;
    fn end(&self) -> usize;
}

impl<T: AsRef<str>> MyTrait for T {
    fn start(&self) -> usize {
        (self.as_ref().chars().nth(0).unwrap() as u8 - 97) as usize
    }

    fn end(&self) -> usize {
        (self.as_ref().chars().last().unwrap() as u8 - 97) as usize
    }
}

impl Solution {
    pub fn minimize_concatenated_length(words: Vec<String>) -> i32 {
        let mut dp = vec![[[-1; 26]; 26]; words.len()];
        dp[0][words[0].start()][words[0].end()] = 0;
        let mut total_len = words[0].len();
        for i in 0..words.len() - 1 {
            let word = &words[i + 1];
            total_len += word.len();
            for start in 0..26 {
                for end in 0..26 {
                    if dp[i][start][end] == -1 {
                        continue;
                    }
                    let this_val = dp[i][start][end];
                    {
                        let next = &mut dp[i + 1][start][word.end()];
                        if end == word.start() {
                            *next = std::cmp::max(this_val + 1, *next);
                        } else {
                            *next = std::cmp::max(this_val, *next);
                        }
                    }
                    {
                        let next = &mut dp[i + 1][word.start()][end];
                        if start == word.end() {
                            *next = std::cmp::max(this_val + 1, *next);
                        } else {
                            *next = std::cmp::max(this_val, *next);
                        }
                    }
                }
            }
        }
        let mut res = -1;
        let res_2d = dp[words.len() - 1];
        for start in 0..26 {
            for end in 0..26 {
                res = std::cmp::max(res, res_2d[start][end]);
            }
        }
        total_len as i32 - res
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

    #[test]
    fn test4() {
        let res = Solution::minimize_concatenated_length(vec![
            "ab".to_owned(),
            "acb".to_owned(),
        ]);
        assert_eq!(res, 5);
    }

    #[test]
    fn test5() {
        let res = Solution::minimize_concatenated_length(vec![
            "abb".to_owned(),
            "ac".to_owned(),
        ]);
        assert_eq!(res, 5);
    }

    #[test]
    fn test6() {
        let res = Solution::minimize_concatenated_length(vec![
            "a".to_owned(),
            "bc".to_owned(),
            "c".to_owned(),
        ]);
        assert_eq!(res, 3);
    }
}
