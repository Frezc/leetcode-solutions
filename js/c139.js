const _ = require('lodash');
/**
 * @param {string} str1
 * @param {string} str2
 * @return {string}
 */
var gcdOfStrings = function(str1, str2) {
  const len1 = getCommon(str1);
  const len2 = getCommon(str2);
  let max = 0;
  for (let i = 0; i < len1.length; i++) {
    for (let j = 0; j < len2.length; j++) {
      if (len1[i] === len2[j] && str1.slice(0, len1[i]) === str2.slice(0, len2[j])) {
        if (len1[i] > max) {
          max = len1[i];
        }
      }
    }
  }
  return str1.slice(0, max);
};

function getCommon(str) {
  const result = [];
  for(let len = 1; len <= str.length;len++) {
    if (str.length % len !== 0) {
      continue;
    }
    const c = str.slice(0, len);
    let valid = true;
    for (let j = 1; j < str.length / len; j++) {
      const c1 = str.slice(j * len, (j+1)*len);
      if (c1 !== c) {
        valid = false;
        break;
      }
    }
    if (valid) {
      result.push(len);
    }
  }
  return result;
}


it('1', () => {
  expect(gcdOfStrings("ABCABC", "ABC")).toEqual("ABC");
  expect(gcdOfStrings("ABABAB", "ABAB")).toEqual("AB");
  expect(gcdOfStrings("LEET", "CODE")).toEqual("");
});


/**
 * @param {number[]} arr1
 * @param {number[]} arr2
 * @return {number[]}
 */
var addNegabinary = function(arr1, arr2) {
  if (_.isEqual(arr1, [0]) && _.isEqual(arr2, [0])) {
    return [0];
  } 
  const pos = binaryAdd(arr1.map((n, i) => i % 2 === 0 ? n : 0), arr2.map((n, i) => i % 2 === 0 ? n : 0));
  const neg = binaryAdd(arr1.map((n, i) => i % 2 !== 0 ? n : 0), arr2.map((n, i) => i % 2 !== 0 ? n : 0));
  const result = binaryAdd(pos, binaryAdd(neg.map((n) => n === 1 ? 0 : 1), [1]));
  result.shift();
  while (result[0] === 0) {
    result.shift();
  }
  if (result.length === 0) {
    result.push(0);
  }
  return result;
};

function binaryAdd(rn1, rn2) {
  const res = [];
  const n1 = rn1.slice().reverse();
  const n2 = rn2.slice().reverse();
  const len = Math.max(n1.length, n2.length);
  let flag = 0;
  for (let i = 0; i < len; i++) {
    const base = (n1[i] || 0) + (n2[i] || 0) + flag;
    flag = Math.floor(base / 2);
    res.push(base % 2);
  }

  if (flag > 0) {
    res.push(1);
  }
  return res.reverse();
}

it('2', () => {
  expect(addNegabinary([1,1,1,1,1], [1,0,1])).toEqual([1,0,0,0,0]);
  expect(addNegabinary([1,1,1,1], [1,0,1])).toEqual([0]);
  expect(addNegabinary([1,1,1,1,1], [1,0,1])).toEqual([1,0,0,0,0]);
});