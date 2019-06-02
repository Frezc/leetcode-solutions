/**
 * [93] Restore IP Addresses
 *
 * Given a string containing only digits, restore it by returning all possible valid IP address combinations.
 * 
 * Example:
 * 
 * 
 * Input: "25525511135"
 * Output: ["255.255.11.135", "255.255.111.35"]
 * 
 * 
 */
/// Similar question to #91.
/// But we use different solution here.
/// This question only request ipv4 checking, so we could consider put 3 "." to the gap of numbers.
/// And check 4 parts is valid ipv4 number.
///
/// Here is some edge cases for this question:
/// 1. "0" is valid.
/// 2. "00", "01", "012" leading "0" is invalid.
/// 3. 0 ~ 255 is valid
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = vec![];
        if s.len() < 4 || s.len() > 12 {
            return result;
        }

        for i in 1..=3 {
            let p1 = &s[..i];
            if !check_part_valid(p1) {
                continue;
            }
            for j in i+1..=i+3 {
                if j > s.len() - 2 {
                    continue;
                }
                let p2 = &s[i..j];
                if !check_part_valid(p2) {
                    continue;
                }
                for k in j+1..=j+3 {
                    if k > s.len() - 1 {
                        continue;
                    }
                    let p3 = &s[j..k];
                    let p4 = &s[k..];
                    if check_part_valid(p3) && check_part_valid(p4) {
                        result.push([p1,p2,p3,p4].join("."));
                    }
                }
            }
        }
        result
    }
}

/// important part
fn check_part_valid(s: &str) -> bool {
    if s.len() > 1 && &s[..1] == "0" {
        return false;
    }
    if s.parse::<u32>().unwrap() > 255 {
        return false;
    }
    true
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_93() {
        assert_vec_eq(&Solution::restore_ip_addresses("25525511135".to_string()), &strvec!["255.255.11.135", "255.255.111.35"])
    }
}
