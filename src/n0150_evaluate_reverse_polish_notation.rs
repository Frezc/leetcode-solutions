/**
 * [150] Evaluate Reverse Polish Notation
 *
 * Evaluate the value of an arithmetic expression in <a href="http://en.wikipedia.org/wiki/Reverse_Polish_notation" target="_blank">Reverse Polish Notation</a>.
 * 
 * Valid operators are +, -, *, /. Each operand may be an integer or another expression.
 * 
 * Note:
 * 
 * 
 * 	Division between two integers should truncate toward zero.
 * 	The given RPN expression is always valid. That means the expression would always evaluate to a result and there won't be any divide by zero operation.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: ["2", "1", "+", "3", "*"]
 * Output: 9
 * Explanation: ((2 + 1) * 3) = 9
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: ["4", "13", "5", "/", "+"]
 * Output: 6
 * Explanation: (4 + (13 / 5)) = 6
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
 * Output: 22
 * Explanation: 
 *   ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
 * = ((10 * (6 / (12 * -11))) + 17) + 5
 * = ((10 * (6 / -132)) + 17) + 5
 * = ((10 * 0) + 17) + 5
 * = (0 + 17) + 5
 * = 17 + 5
 * = 22
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /// 使用一个栈来保存数字，遇到操作符取top2的数字计算即可，注意顺序。
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }

        let mut stack = Vec::with_capacity(tokens.len());
        for token in tokens {
            match &token[..] {
                "+" => {
                    let val = stack.pop().unwrap() + stack.pop().unwrap();
                    stack.push(val);
                },
                "-" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    let val = b - a;
                    stack.push(val);
                },
                "*" => {
                    let val = stack.pop().unwrap() * stack.pop().unwrap();
                    stack.push(val);
                },
                "/" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    let val = b / a;
                    stack.push(val);
                },
                num => {
                    stack.push(num.parse::<i32>().unwrap());
                }
            }
        }

        stack.pop().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_150() {
        assert_eq!(Solution::eval_rpn(strvec![]), 0);
        assert_eq!(Solution::eval_rpn(strvec!["2", "1", "+", "3", "*"]), 9);
        assert_eq!(Solution::eval_rpn(strvec!["4", "13", "5", "/", "+"]), 6);
        assert_eq!(Solution::eval_rpn(strvec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]), 22);
    }
}
