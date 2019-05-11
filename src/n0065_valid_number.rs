/**
 * [65] Valid Number
 *
 * Validate if a given string can be interpreted as a decimal number.
 * 
 * Some examples:<br />
 * "0" => true<br />
 * " 0.1 " => true<br />
 * "abc" => false<br />
 * "1 a" => false<br />
 * "2e10" => true<br />
 * " -90e3   " => true<br />
 * " 1e" => false<br />
 * "e3" => false<br />
 * " 6e-1" => true<br />
 * " 99e2.5 " => false<br />
 * "53.5e93" => true<br />
 * " --6 " => false<br />
 * "-+3" => false<br />
 * "95a54e53" => false
 * 
 * Note: It is intended for the problem statement to be ambiguous. You should gather all requirements up front before implementing one. However, here is a list of characters that can be in a valid decimal number:
 * 
 * 
 * 	Numbers 0-9
 * 	Exponent - "e"
 * 	Positive/negative sign - "+"/"-"
 * 	Decimal point - "."
 * 
 * 
 * Of course, the context of these characters also matters in the input.
 * 
 * Update (2015-02-10):<br />
 * The signature of the C++ function had been updated. If you still see your function signature accepts a const char * argument, please click the reload button to reset your code definition.
 * 
 */
/// First check all test cases.
/// Write a simple parser with bnf below
/// ```bnf
/// <exp> ::= [<ws>] <number> [<ws>]
/// <number> ::= <decimals> ['e' <nodecimals>]
/// <decimals> ::= ['+' | '-'] (<digit> ['.' [<digit>]] | '.' <digit>)
/// <nodecimals>::= ['+' | '-'] <digit>
/// <digit>  ::= ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') [<digit>]
/// <ws>         ::= ' ' [<ws>]
/// ```
pub struct Solution {}

// submission codes start here
use std::iter::Peekable;
use std::str::Chars;

impl Solution {
    pub fn is_number(s: String) -> bool {
        Parser::new(&s).parse().is_ok()
    }
}

struct Parser<'a> {
    iter: Peekable<Chars<'a>>
}

impl<'a> Parser<'a> {
    fn new(s: &str) -> Parser {
        Parser {
            iter: s.chars().peekable()
        }
    }

    fn parse(&mut self) -> Result<(), ()> {
        self.skip_ws();
        self.parse_number()?;
        self.skip_ws();

        if let None = self.iter.peek() {
            Ok(())
        } else {
            Err(())
        }
    }

    fn skip_ws(&mut self) {
        while let Some(' ') = self.iter.peek() {
            self.iter.next();
        }
    }

    fn parse_number(&mut self) -> Result<(), ()> {
        self.parse_decimals()?;
        if let Some(&'e') = self.iter.peek() {
            self.iter.next();
            self.parse_nodecimals()?;
        }
        Ok(())
    }

    fn parse_decimals(&mut self) -> Result<(), ()> {
        if let Some(op) = self.iter.peek() {
            match op {
                '+' | '-' => { self.iter.next(); },
                _ => {},
            }
        }
        if let Some(c) = self.iter.peek() {
            match c {
                '0'...'9' => {
                    self.parse_digits()?;
                    if let Some('.') = self.iter.peek() {
                        self.iter.next();
                        // it is ok even failed
                        self.parse_digits().ok();
                    }
                }
                '.' => {
                    self.iter.next();
                    self.parse_digits()?;
                }
                _ => return Err(())
            }
            Ok(())
        } else {
            Err(())
        }
    }

    fn parse_nodecimals(&mut self) -> Result<(), ()> {
        if let Some(op) = self.iter.peek() {
            match op {
                '+' | '-' => { self.iter.next(); },
                _ => {},
            }
        }
        if let Some(c) = self.iter.peek() {
            match c {
                '0'...'9' => {
                    self.parse_digits()?;
                }
                _ => return Err(())
            }
            Ok(())
        } else {
            Err(())
        }
    }

    fn parse_digits(&mut self) -> Result<(), ()> {
        let mut parsed = false;
        while let Some('0'...'9') = self.iter.peek() {
            if !parsed { parsed = true; }
            self.iter.next();
        }
        if parsed { Ok(()) } else { Err(()) }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_65() {
        assert_eq!(Solution::is_number("123".to_string()), true);
        assert_eq!(Solution::is_number(" 123 ".to_string()), true);
        assert_eq!(Solution::is_number("0".to_string()), true);
        assert_eq!(Solution::is_number("0123".to_string()), true);
        assert_eq!(Solution::is_number("00".to_string()), true);
        assert_eq!(Solution::is_number("-10".to_string()), true);
        assert_eq!(Solution::is_number("-0".to_string()), true);
        assert_eq!(Solution::is_number("123.5".to_string()), true);
        assert_eq!(Solution::is_number("123.00000".to_string()), true);
        assert_eq!(Solution::is_number("-500.777".to_string()), true);
        assert_eq!(Solution::is_number("0.0000001".to_string()), true);
        assert_eq!(Solution::is_number("0.00000".to_string()), true);
        assert_eq!(Solution::is_number("0.".to_string()), true);
        assert_eq!(Solution::is_number("00.5".to_string()), true);
        assert_eq!(Solution::is_number("123e1".to_string()), true);
        assert_eq!(Solution::is_number("1.23e10".to_string()), true);
        assert_eq!(Solution::is_number("0.5e-10".to_string()), true);
        assert_eq!(Solution::is_number("1.0e4.5".to_string()), false);
        assert_eq!(Solution::is_number("0.5e04".to_string()), true);
        assert_eq!(Solution::is_number("12 3".to_string()), false);
        assert_eq!(Solution::is_number("1a3".to_string()), false);
        assert_eq!(Solution::is_number("".to_string()), false);
        assert_eq!(Solution::is_number("      ".to_string()), false);
        assert_eq!(Solution::is_number(".1".to_string()), true);
        assert_eq!(Solution::is_number(".".to_string()), false);
        assert_eq!(Solution::is_number("2e0".to_string()), true);
        assert_eq!(Solution::is_number("+.8".to_string()), true);
        assert_eq!(Solution::is_number(" 005047e+6".to_string()), true);
    }
}
