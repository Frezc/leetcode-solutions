/**
 * [68] Text Justification
 *
 * Given an array of words and a width maxWidth, format the text such that each line has exactly maxWidth characters and is fully (left and right) justified.
 * 
 * You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces ' ' when necessary so that each line has exactly maxWidth characters.
 * 
 * Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line do not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.
 * 
 * For the last line of text, it should be left justified and no extra space is inserted between words.
 * 
 * Note:
 * 
 * 
 * 	A word is defined as a character sequence consisting of non-space characters only.
 * 	Each word's length is guaranteed to be greater than 0 and not exceed maxWidth.
 * 	The input array words contains at least one word.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input:
 * words = ["This", "is", "an", "example", "of", "text", "justification."]
 * maxWidth = 16
 * Output:
 * [
 *    "This    is    an",
 *    "example  of text",
 *    "justification.  "
 * ]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input:
 * words = ["What","must","be","acknowledgment","shall","be"]
 * maxWidth = 16
 * Output:
 * [
 *   "What   must   be",
 *   "acknowledgment  ",
 *   "shall be        "
 * ]
 * Explanation: Note that the last line is "shall be    " instead of "shall     be",
 *              because the last line must be left-justified instead of fully-justified.
 *              Note that the second line is also left-justified becase it contains only one word.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input:
 * words = ["Science","is","what","we","understand","well","enough","to","explain",
 *          "to","a","computer.","Art","is","everything","else","we","do"]
 * maxWidth = 20
 * Output:
 * [
 *   "Science  is  what we",
 *   "understand      well",
 *   "enough to explain to",
 *   "a  computer.  Art is",
 *   "everything  else  we",
 *   "do                  "
 * ]
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut result = vec![];
        let mut temp: Vec<String> = vec![];
        let mut len = 0;
        let max_width = max_width as usize;
        for word in words.into_iter() {
            let word_len = word.chars().count();
            let predicted_len = if len == 0 { len + word_len } else { len + word_len + 1 };
            if predicted_len > max_width {
                let rest_space = max_width - len;
                let mut words_with_space = Vec::with_capacity(temp.len() * 2);
                if temp.len() > 1 {
                    let space_each = rest_space / (temp.len() - 1) + 1;
                    let mut addition_space = rest_space % (temp.len() - 1);
                    for (i, w) in temp.iter().enumerate() {
                        if i != 0 {
                            words_with_space.push(vec![String::from(" "); space_each + if addition_space > 0 { addition_space -= 1; 1 } else { 0 }].concat());
                        }
                        words_with_space.push(w.to_owned());
                    }
                } else {
                    words_with_space.push(temp[0].to_owned());
                    words_with_space.push(vec![String::from(" "); rest_space].concat());
                }

                result.push(words_with_space.concat());
                temp.clear();
                len = 0;
            }
            if temp.len() != 0 {
                len += 1;
            }
            temp.push(word);
            len += word_len;
        }
        if len < max_width {
            temp.push(vec![String::from(" "); max_width - len - 1].concat());
        }
        result.push(temp.join(" "));
        result
    }
}


// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_68() {
        assert_eq!(Solution::full_justify(strvec!["This", "is", "an", "example", "of", "text", "justification."], 16), strvec![
           "This    is    an",
           "example  of text",
           "justification.  "
        ]);
        assert_eq!(Solution::full_justify(strvec!["What","must","be","acknowledgment","shall","be"], 16), strvec![
           "What   must   be",
           "acknowledgment  ",
           "shall be        "
        ]);
        assert_eq!(Solution::full_justify(strvec!["Science","is","what","we","understand","well","enough","to","explain",
         "to","a","computer.","Art","is","everything","else","we","do"], 20), strvec![
           "Science  is  what we",
           "understand      well",
           "enough to explain to",
           "a  computer.  Art is",
           "everything  else  we",
           "do                  "
        ]);
        assert_eq!(Solution::full_justify(strvec!["abcdefghijklmnop"], 16), strvec![
           "abcdefghijklmnop"
        ]);
    }
}
