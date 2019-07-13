/**
 * [123] Best Time to Buy and Sell Stock III
 *
 * Say you have an array for which the i^th element is the price of a given stock on day i.
 * 
 * Design an algorithm to find the maximum profit. You may complete at most two transactions.
 * 
 * Note: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
 * 
 * Example 1:
 * 
 * 
 * Input: [3,3,5,0,0,3,1,4]
 * Output: 6
 * Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
 *              Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
 * 
 * Example 2:
 * 
 * 
 * Input: [1,2,3,4,5]
 * Output: 4
 * Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
 *              Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
 *              engaging multiple transactions at the same time. You must sell before buying again.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transaction is done, i.e. max profit = 0.
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 { return 0 }
        let mut max = 0;

        for i in 2..prices.len()-1 {
            let profit = max_profit_1(&prices[0..i]) + max_profit_1(&prices[i..]);
            max = max.max(profit);
        }
        max.max(max_profit_1(&prices))
    }
}

pub fn max_profit_1(prices: &[i32]) -> i32 {
    // the lowest price up to someday
    let mut lowest = vec![0i32; prices.len()];
    // the highest price on and after someday
    let mut highest = vec![0i32; prices.len()];
    let days = prices.len();

    for i in 0..prices.len() {
        if i == 0 {
            lowest[0] = prices[0];
            highest[days - 1] = prices[days - 1];
        } else {
            lowest[i] = prices[i].min(lowest[i - 1]);
            highest[days - i - 1] = prices[days - i - 1].max(highest[days - i]);
        }
    }

    // calculate best profit to everyday, and find maximum
    lowest.iter()
        .zip(highest.iter())
        .fold(0, |max_profit, (lowest, highest)| {
            max_profit.max(highest - lowest)
        })
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_123() {
        assert_eq!(Solution::max_profit(vec![3,3,5,0,0,3,1,4]), 6);
        assert_eq!(Solution::max_profit(vec![1,2,3,4,5]), 4);
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
        assert_eq!(Solution::max_profit(vec![1,2,4,2,5,7,2,4,9,0]), 13);
        assert_eq!(Solution::max_profit(vec![]), 0);
    }
}
