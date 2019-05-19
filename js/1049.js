/**
 * @param {number[]} stones
 * @return {number}
 * WA
 */
let lastStoneWeightII = function (stones) {
  while (stones.length > 1) {
    let min = Infinity;
    let ij = [0, 1];
    for (let i = 0; i < stones.length - 1; i++) {
      for (let j = i + 1; j < stones.length; j++) {
        let si = stones[i];
        let sj = stones[j];
        stones[i] -= Math.min(si, sj);
        stones[j] -= Math.min(si, sj);
        console.log('input', stones)
        let variance1 = variance(stones.filter(a => a > 0));
        stones[i] += Math.min(si, sj);
        stones[j] += Math.min(si, sj);
        console.log('vari', variance1)
        if (variance1 < min) {
          min = variance1;
          ij = [i, j];
        }
      }
    }
    const minn = Math.min(stones[ij[0]], stones[ij[1]]);
    stones[ij[0]] -= minn;
    stones[ij[1]] -= minn;
    stones = stones.filter(a => a>0);
  }
  return stones[0] || 0;
};

function variance(numbers) {
  let mean = 0;
  let sum = 0;
  for (let i = 0; i < numbers.length; i++) {
    sum += numbers[i];
  }
  mean = sum / numbers.length;
  sum = 0;
  for (let i = 0; i < numbers.length; i++) {
    sum += Math.pow(numbers[i] - mean, 2);
  }
  return sum / numbers.length;
};

it('1049', () => {
  expect(lastStoneWeightII([2, 7, 4, 1, 8, 1])).toEqual(1);
  expect(lastStoneWeightII([2, 1, 2, 2, 2])).toEqual(1);
  expect(lastStoneWeightII([1, 1, 4, 2, 2])).toEqual(0);
  expect(lastStoneWeightII([31, 26, 33, 21, 40])).toEqual(5);
})