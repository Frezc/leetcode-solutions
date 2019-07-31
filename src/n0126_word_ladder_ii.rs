
/**
 * [126] Word Ladder II
 *
 * Given two words (beginWord and endWord), and a dictionary's word list, find all shortest transformation sequence(s) from beginWord to endWord, such that:
 * 
 * <ol>
 * 	Only one letter can be changed at a time
 * 	Each transformed word must exist in the word list. Note that beginWord is not a transformed word.
 * </ol>
 * 
 * Note:
 * 
 * 
 * 	Return an empty list if there is no such transformation sequence.
 * 	All words have the same length.
 * 	All words contain only lowercase alphabetic characters.
 * 	You may assume no duplicates in the word list.
 * 	You may assume beginWord and endWord are non-empty and are not the same.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input:
 * beginWord = "hit",
 * endWord = "cog",
 * wordList = ["hot","dot","dog","lot","log","cog"]
 * 
 * Output:
 * [
 *   ["hit","hot","dot","dog","cog"],
 *   ["hit","hot","lot","log","cog"]
 * ]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input:
 * beginWord = "hit"
 * endWord = "cog"
 * wordList = ["hot","dot","dog","lot","log"]
 * 
 * Output: []
 * 
 * Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.
 * 
 * 
 * 
 * 
 * 
 */
/// todo
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        if word_list.iter().find(|&s| s == &end_word).is_none() {
            return vec![];
        }

        let full_words = [vec![begin_word.clone()], word_list].concat();
        let map = construct_ladder_map(&full_words);
        let mut queue = VecDeque::new();
        queue.push_back(vec![begin_word]);
        while !queue.is_empty() {
            let mut temp = vec![];
            // if found end_word
            let mut flag = false;
            while let Some(words) = queue.pop_front() {
                let trans = map.get(words.last().unwrap()).unwrap();
                for &tran in trans {
                    if tran == &end_word {
                        flag = true;
                        temp.push([words.clone(), vec![tran.clone()]].concat());
                    } else if !flag && !words.contains(tran) {
                        temp.push([words.clone(), vec![tran.clone()]].concat());
                    }
                }
            }

            if flag {
                return temp.into_iter().filter(|paths| paths.last().unwrap() == &end_word).collect();
            }
            queue.extend(temp);
        }

        vec![]
    }
}

fn calculate_distance(w1: &str, w2: &str) -> usize {
    let mut count = 0;
    for (c1, c2) in w1.chars().zip(w2.chars()) {
        if c1 != c2 {
            count += 1;
        }
    }
    count
}

fn construct_ladder_map(word_list: &Vec<String>) -> HashMap<&String, Vec<&String>> {
    let mut result = HashMap::new();
    for word1 in word_list {
        let mut target = vec![];
        for word2 in word_list {
            if word1 != word2 && is_transformed(&word1, &word2) {
                target.push(word2);
            }
        }
        result.insert(word1, target);
    }

    result
}

fn is_transformed(s1: &str, s2: &str) -> bool {
    let mut count = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            count += 1;
        }

        if count > 1 {
            return false;
        }
    }

    return true;
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_126() {
        assert_eq!(Solution::find_ladders("hit".to_string(), "cog".to_string(), strvec!["hot","dot","dog","lot","log","cog"]), vec![
            strvec!["hit","hot","dot","dog","cog"],
            strvec!["hit","hot","lot","log","cog"]
        ]);
        let empty: Vec<Vec<String>> = vec![];
        assert_eq!(Solution::find_ladders("hit".to_string(), "mmm".to_string(), strvec!["hot","dot","dog","lot","log","cog"]), empty);

    }

    #[test]
    fn test_126_2() {
        assert_eq!(Solution::find_ladders("hit".to_string(), "hot".to_string(), strvec!["hot","dot","dog","lot","log","cog"]), vec![
            strvec!["hit","hot"]
        ]);
    }

    #[test]
    fn test_126_tle() {
        assert_eq!(Solution::find_ladders("qa".to_string(), "sq".to_string(), strvec!["si","go","se","cm","so","ph","mt","db","mb","sb","kr","ln","tm","le","av","sm","ar","ci","ca","br","ti","ba","to","ra","fa","yo","ow","sn","ya","cr","po","fe","ho","ma","re","or","rn","au","ur","rh","sr","tc","lt","lo","as","fr","nb","yb","if","pb","ge","th","pm","rb","sh","co","ga","li","ha","hz","no","bi","di","hi","qa","pi","os","uh","wm","an","me","mo","na","la","st","er","sc","ne","mn","mi","am","ex","pt","io","be","fm","ta","tb","ni","mr","pa","he","lr","sq","ye"]), vec2![["qa", "ca", "cm", "sm", "sq"], ["qa", "ca", "ci", "si", "sq"], ["qa", "ca", "cr", "sr", "sq"], ["qa", "ca", "co", "so", "sq"], ["qa", "ba", "br", "sr", "sq"], ["qa", "ba", "bi", "si", "sq"], ["qa", "ba", "be", "se", "sq"], ["qa", "ra", "re", "se", "sq"], ["qa", "ra", "rn", "sn", "sq"], ["qa", "ra", "rh", "sh", "sq"], ["qa", "ra", "rb", "sb", "sq"], ["qa", "fa", "fe", "se", "sq"], ["qa", "fa", "fr", "sr", "sq"], ["qa", "fa", "fm", "sm", "sq"], ["qa", "ya", "yo", "so", "sq"], ["qa", "ya", "yb", "sb", "sq"], ["qa", "ya", "ye", "se", "sq"], ["qa", "ma", "mt", "st", "sq"], ["qa", "ma", "mb", "sb", "sq"], ["qa", "ma", "me", "se", "sq"], ["qa", "ma", "mo", "so", "sq"], ["qa", "ma", "mn", "sn", "sq"], ["qa", "ma", "mi", "si", "sq"], ["qa", "ma", "mr", "sr", "sq"], ["qa", "ga", "go", "so", "sq"], ["qa", "ga", "ge", "se", "sq"], ["qa", "ha", "ho", "so", "sq"], ["qa", "ha", "hi", "si", "sq"], ["qa", "ha", "he", "se", "sq"], ["qa", "na", "nb", "sb", "sq"], ["qa", "na", "no", "so", "sq"], ["qa", "na", "ne", "se", "sq"], ["qa", "na", "ni", "si", "sq"], ["qa", "la", "ln", "sn", "sq"], ["qa", "la", "le", "se", "sq"], ["qa", "la", "lt", "st", "sq"], ["qa", "la", "lo", "so", "sq"], ["qa", "la", "li", "si", "sq"], ["qa", "la", "lr", "sr", "sq"], ["qa", "ta", "tm", "sm", "sq"], ["qa", "ta", "ti", "si", "sq"], ["qa", "ta", "to", "so", "sq"], ["qa", "ta", "tc", "sc", "sq"], ["qa", "ta", "th", "sh", "sq"], ["qa", "ta", "tb", "sb", "sq"], ["qa", "pa", "ph", "sh", "sq"], ["qa", "pa", "po", "so", "sq"], ["qa", "pa", "pb", "sb", "sq"], ["qa", "pa", "pm", "sm", "sq"], ["qa", "pa", "pi", "si", "sq"], ["qa", "pa", "pt", "st", "sq"]]);
    }

    #[test]
    fn test_126_mle() {
        assert_eq!(Solution::find_ladders("cet".to_string(), "ism".to_string(), strvec!["kid","tag","pup","ail","tun","woo","erg","luz","brr","gay","sip","kay","per","val","mes","ohs","now","boa","cet","pal","bar","die","war","hay","eco","pub","lob","rue","fry","lit","rex","jan","cot","bid","ali","pay","col","gum","ger","row","won","dan","rum","fad","tut","sag","yip","sui","ark","has","zip","fez","own","ump","dis","ads","max","jaw","out","btu","ana","gap","cry","led","abe","box","ore","pig","fie","toy","fat","cal","lie","noh","sew","ono","tam","flu","mgm","ply","awe","pry","tit","tie","yet","too","tax","jim","san","pan","map","ski","ova","wed","non","wac","nut","why","bye","lye","oct","old","fin","feb","chi","sap","owl","log","tod","dot","bow","fob","for","joe","ivy","fan","age","fax","hip","jib","mel","hus","sob","ifs","tab","ara","dab","jag","jar","arm","lot","tom","sax","tex","yum","pei","wen","wry","ire","irk","far","mew","wit","doe","gas","rte","ian","pot","ask","wag","hag","amy","nag","ron","soy","gin","don","tug","fay","vic","boo","nam","ave","buy","sop","but","orb","fen","paw","his","sub","bob","yea","oft","inn","rod","yam","pew","web","hod","hun","gyp","wei","wis","rob","gad","pie","mon","dog","bib","rub","ere","dig","era","cat","fox","bee","mod","day","apr","vie","nev","jam","pam","new","aye","ani","and","ibm","yap","can","pyx","tar","kin","fog","hum","pip","cup","dye","lyx","jog","nun","par","wan","fey","bus","oak","bad","ats","set","qom","vat","eat","pus","rev","axe","ion","six","ila","lao","mom","mas","pro","few","opt","poe","art","ash","oar","cap","lop","may","shy","rid","bat","sum","rim","fee","bmw","sky","maj","hue","thy","ava","rap","den","fla","auk","cox","ibo","hey","saw","vim","sec","ltd","you","its","tat","dew","eva","tog","ram","let","see","zit","maw","nix","ate","gig","rep","owe","ind","hog","eve","sam","zoo","any","dow","cod","bed","vet","ham","sis","hex","via","fir","nod","mao","aug","mum","hoe","bah","hal","keg","hew","zed","tow","gog","ass","dem","who","bet","gos","son","ear","spy","kit","boy","due","sen","oaf","mix","hep","fur","ada","bin","nil","mia","ewe","hit","fix","sad","rib","eye","hop","haw","wax","mid","tad","ken","wad","rye","pap","bog","gut","ito","woe","our","ado","sin","mad","ray","hon","roy","dip","hen","iva","lug","asp","hui","yak","bay","poi","yep","bun","try","lad","elm","nat","wyo","gym","dug","toe","dee","wig","sly","rip","geo","cog","pas","zen","odd","nan","lay","pod","fit","hem","joy","bum","rio","yon","dec","leg","put","sue","dim","pet","yaw","nub","bit","bur","sid","sun","oil","red","doc","moe","caw","eel","dix","cub","end","gem","off","yew","hug","pop","tub","sgt","lid","pun","ton","sol","din","yup","jab","pea","bug","gag","mil","jig","hub","low","did","tin","get","gte","sox","lei","mig","fig","lon","use","ban","flo","nov","jut","bag","mir","sty","lap","two","ins","con","ant","net","tux","ode","stu","mug","cad","nap","gun","fop","tot","sow","sal","sic","ted","wot","del","imp","cob","way","ann","tan","mci","job","wet","ism","err","him","all","pad","hah","hie","aim","ike","jed","ego","mac","baa","min","com","ill","was","cab","ago","ina","big","ilk","gal","tap","duh","ola","ran","lab","top","gob","hot","ora","tia","kip","han","met","hut","she","sac","fed","goo","tee","ell","not","act","gil","rut","ala","ape","rig","cid","god","duo","lin","aid","gel","awl","lag","elf","liz","ref","aha","fib","oho","tho","her","nor","ace","adz","fun","ned","coo","win","tao","coy","van","man","pit","guy","foe","hid","mai","sup","jay","hob","mow","jot","are","pol","arc","lax","aft","alb","len","air","pug","pox","vow","got","meg","zoe","amp","ale","bud","gee","pin","dun","pat","ten","mob"]), vec![
            strvec!["hit","hot"]
        ]);
    }
}
