/**
 * @param {number} d
 * @param {number} low
 * @param {number} high
 * @return {number}
 */
var digitsCount = function(d, low, high) {
  return calc(high, d) - calc(low-1,d);
};

function calc(num, d) {
  if (num < d) return 0;
  if (num == 0 && d == num) return 1;
  const s = String(num);
  let count = 0;
  for (let i = 0; i < s.length;i++) {
    const n = parseInt(s[i], 10);
    let base = 0;
    if (i < s.length - 1 && d > 0 && n >= d) {
      base = (n == d ? parseInt(s.slice(i+1), 10) + 1 : 10);
    }
    if (i > 0) {
      base = (base || 1) * ((parseInt(s.slice(0, i), 10) || 0) + (n >= d ? 1 : 0));
    }

    count += base;
  }
  return count;
}

it('1058', () => {
  expect(digitsCount(1,1,13)).toEqual(6);
  expect(digitsCount(3,100,250)).toEqual(35);
  expect(digitsCount(0,1,10)).toEqual(1);
})