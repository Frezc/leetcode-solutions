/**
 * @param {string} S
 * @return {string}
 */
var longestDupSubstring = function(S) {
  let result = '';
  for(let i = 0; i < S.length; i++) {
    let j = i + 1;
    while (j < S.length) {
      if (S[j] === S[i]) {
        const len = maxDupLen(S, i, j);
        if (len > result.length) {
          result = S.slice(i, i + len);
        }

        j += len;
      } else {
        j++;
      }
    }
  }
  return result;
};

function maxDupLen(S, s1, s2) {
  let len = 0;
  for(let i = 0; s1 + i < S.length && s2 + i < S.length; i++) {
    if (S[s1+i] === S[s2+i]) {
      len++;
    } else {
      break;
    }
  }
  return len;
}

it('test', () => {
  expect(longestDupSubstring('banana')).toEqual('ana');
  expect(longestDupSubstring('abcd')).toEqual('');
})