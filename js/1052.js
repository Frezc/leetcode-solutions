/**
 * @param {number[]} customers
 * @param {number[]} grumpy
 * @param {number} X
 * @return {number}
 */
var maxSatisfied = function(customers, grumpy, X) {
  let max = 0;
  for (let i = 0; i < grumpy.length - X + 1; i++) {
    const newGrumpy = grumpy.slice();
    for (let j = i; j < i + X; j++) {
      newGrumpy[j] = 0;
    }
    const s = satisfied(customers, newGrumpy);
    if (s > max) {
      max = s;
    }
  }
  return max;
};

function satisfied(customers, grumpy) {
  let sum = 0;
  for (let i = 0; i < customers.length; i++) {
    if (grumpy[i] == 0) {
      sum+=customers[i];
    }
  }
  return sum;
}

it('1052', () => {
  expect(maxSatisfied([1,0,1,2,1,1,7,5], [0,1,0,1,0,1,0,1], 3)).toEqual(16);
})