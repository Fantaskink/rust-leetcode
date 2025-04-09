pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|x| *x != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut vec1 = vec![3, 2, 2, 3];
        assert_eq!(2, Solution::remove_element(&mut vec1, 3));
        assert_eq!(vec![2, 2], vec1);

        let mut vec2 = vec![0,1,2,2,3,0,4,2];
        assert_eq!(5, Solution::remove_element(&mut vec2, 2));
        assert_eq!(vec![0,1,3,0,4], vec2);
    }
}
