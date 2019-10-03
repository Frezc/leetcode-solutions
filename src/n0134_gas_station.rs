/**
 * [134] Gas Station
 *
 * There are N gas stations along a circular route, where the amount of gas at station i is gas[i].
 * 
 * You have a car with an unlimited gas tank and it costs cost[i] of gas to travel from station i to its next station (i+1). You begin the journey with an empty tank at one of the gas stations.
 * 
 * Return the starting gas station's index if you can travel around the circuit once in the clockwise direction, otherwise return -1.
 * 
 * Note:
 * 
 * 
 * 	If there exists a solution, it is guaranteed to be unique.
 * 	Both input arrays are non-empty and have the same length.
 * 	Each element in the input arrays is a non-negative integer.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: 
 * gas  = [1,2,3,4,5]
 * cost = [3,4,5,1,2]
 * 
 * Output: 3
 * 
 * Explanation:
 * Start at station 3 (index 3) and fill up with 4 unit of gas. Your tank = 0 + 4 = 4
 * Travel to station 4. Your tank = 4 - 1 + 5 = 8
 * Travel to station 0. Your tank = 8 - 2 + 1 = 7
 * Travel to station 1. Your tank = 7 - 3 + 2 = 6
 * Travel to station 2. Your tank = 6 - 4 + 3 = 5
 * Travel to station 3. The cost is 5. Your gas is just enough to travel back to station 3.
 * Therefore, return 3 as the starting index.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 
 * gas  = [2,3,4]
 * cost = [3,4,3]
 * 
 * Output: -1
 * 
 * Explanation:
 * You can't start at station 0 or 1, as there is not enough gas to travel to the next station.
 * Let's start at station 2 and fill up with 4 unit of gas. Your tank = 0 + 4 = 4
 * Travel to station 0. Your tank = 4 - 3 + 2 = 3
 * Travel to station 1. Your tank = 3 - 3 + 3 = 3
 * You cannot travel back to station 2, as it requires 4 unit of gas but you only have 3.
 * Therefore, you can't travel around the circuit once no matter where you start.
 * 
 * 
 */
/// The most important idea is:
/// If B is the **first** station tank cannot reach when tank start from A,
/// stations between A & B also cannot reach B.
/// (Think about a C between A to B. Obviously A can reach C, because B is the first station A cannot reach. Thus C cannot reach B)
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut i: usize = 0;
        let len = gas.len();

        loop {
            let s = i;
            let mut gas_v = 0;
            while gas_v >= 0 {
                gas_v += gas[i] - cost[i];
                i = (i + 1) % len;

                if i == s {
                    if gas_v >= 0 {
                        return i as i32;
                    }
                }
            }

            if i <= s {
                break;
            }
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_134() {
        assert_eq!(Solution::can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]), 3);
        assert_eq!(Solution::can_complete_circuit(vec![2,3,4], vec![3,4,3]), -1);

    }
}
