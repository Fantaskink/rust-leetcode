pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut vec1 = vec![1, 1, 2];
        assert_eq!(2, Solution::remove_duplicates(&mut vec1));
        assert_eq!(vec![1, 2], vec1);

        let mut vec2 = vec![0, 0, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, Solution::remove_duplicates(&mut vec2));
        assert_eq!(vec![0, 1, 2, 3, 4], vec2);
    }
}