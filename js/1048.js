/**
 * @param {string[]} words
 * @return {number}
 */
var longestStrChain = function(words) {
  words.sort((a, b) => a.length - b.length);
  const dp = [];
  let max = 0;
  for (let i = 0; i < words.length; i++) {
    max = Math.max(strChanin(dp, words, i), max);
  }
  return max;
};

function strChanin(dp, words, startIndex) {
  if (startIndex === words.length - 1) {
    return 1;
  }

  if (dp[startIndex]) {
    return dp[startIndex];
  }

  let max = 1;
  for (let i = startIndex + 1; i < words.length; i++) {
    if (isPredecessor(words[startIndex], words[i])) {
      let c = strChanin(dp, words, i);
      if (c + 1 > max) {
        max = c + 1;
      }
    }
  }
  dp[startIndex] = max;
  return dp[startIndex];
}

function isPredecessor(str1, str2) {
  if (str1.length + 1 !== str2.length) {
    return false;
  }
  let dif = false;
  for(let i = 0; i < str1.length; i++) {
    if (str1[i] !== str2[dif ? i + 1 : i]) {
      if (dif) {
        return false;
      } else {
        // edge case: 'x' & 'pu'
        if (str1[i] !== str2[i+1]) {
          return false;
        }
        dif = true;
      }
    }
  }

  return true;
}

it('1048', () => {
  expect(longestStrChain(["a","b","ba","bca","bda","bdca"])).toEqual(4);
  expect(longestStrChain(["a","b"])).toEqual(1);
  expect(longestStrChain(["ksqvsyq","ks","kss","czvh","zczpzvdhx","zczpzvh","zczpzvhx","zcpzvh","zczvh","gr","grukmj","ksqvsq","gruj","kssq","ksqsq","grukkmj","grukj","zczpzfvdhx","gru"])).toEqual(7);
  expect(longestStrChain(["qyssedya","pabouk","mjwdrbqwp","vylodpmwp","nfyqeowa","pu","paboukc","qssedya","lopmw","nfyqowa","vlodpmw","mwdrqwp","opmw","qsda","neo","qyssedhyac","pmw","lodpmw","mjwdrqwp","eo","nfqwa","pabuk","nfyqwa","qssdya","qsdya","qyssedhya","pabu","nqwa","pabqoukc","pbu","mw","vlodpmwp","x","xr"])).toEqual(8);
})