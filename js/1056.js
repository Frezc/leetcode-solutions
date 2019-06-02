/**
 * @param {string} text
 * @param {string[]} words
 * @return {number[][]}
 */
var indexPairs = function(text, words) {
  words.sort((a,b) => a.length - b.length);
  const result = [];
  for (let i = 0; i < text.length; i++) {
    const t = text.slice(i);
    for (const word of words) {
      if (t.startsWith(word)) {
        result.push([i, i+word.length-1]);
      }
    }
  }
  return result;
};

it('1055', () => {
  expect(indexPairs("thestoryofleetcodeandme", ["story","fleet","leetcode"])).toEqual([[3,7],[9,13],[10,17]]);
  expect(indexPairs("ababa", ["aba","ab"])).toEqual([[0,1],[0,2],[2,3],[2,4]]);
})