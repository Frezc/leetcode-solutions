/**
 * @param {number[]} stones
 * @return {number}
 */
var lastStoneWeight = function(stones) {
  if (stones.length === 1) {
    return stones[0];
  }

  while(stones.length > 1) {
    let max = 0, max2 = 0;
    let maxi = 0, max2i = 1;
    for (let i = 0; i < stones.length; i++) {
      if (stones[i] > max) {
        max2 = max;
        max2i = maxi;
        max = stones[i];
        maxi = i;
      } else if (stones[i] > max2) {
        max2 = stones[i];
        max2i = i;
      }
    }

    stones[maxi] -= max2;
    stones[max2i] -= max2;

    stones = stones.filter((s) => s > 0);
  }

  return stones[0] || 0;
};

it('1046', () => {
  expect(lastStoneWeight([2,7,4,1,8,1])).toEqual(1);
})