pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut low_price = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
            if price < low_price {
                low_price = price;
            } else if price - low_price > max_profit {
                max_profit = price - low_price;
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        let prices1 = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(5, Solution::max_profit(prices1));

        let prices2 = vec![7, 6, 4, 3, 1];
        assert_eq!(0, Solution::max_profit(prices2));

        let prices3 = vec![2, 4, 1];
        assert_eq!(2, Solution::max_profit(prices3));
    }
}
