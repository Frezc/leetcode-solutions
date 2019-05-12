/**
 * @param {number[]} A
 * @return {number}
 */
var minScoreTriangulation = function(A) {
  const s = new Solution(A);
  const result = s.solve(0, A.length - 1);
  return result;
};

class Solution {
  constructor(A) {
    this.A = A;
    this.dp = [];
    for (let i = 0; i < A.length; i++) {
      this.dp.push([]);
    }
  }

  solve(start, end) {
    if (!this.dp[start][end]) {
      if (end - start <= 1) {
        return 0;
      }

      let min = Infinity;
      for (let i = start + 1; i < end; i++) {
        const score = this.solve(start, i) + this.solve(i, end) + this.A[start]*this.A[end]*this.A[i];
        if (score < min) {
          min = score;
        }
      }
      this.dp[start][end] = min;
    }
    return this.dp[start][end];
  }
}

it('1039', () => {
  const arr = [3,2,1];
  arr.sort();
  expect(arr).toEqual([1,2,3]);
  expect(minScoreTriangulation([1,2,3])).toBe(6);
  expect(minScoreTriangulation([3,7,4,5])).toBe(144);
  expect(minScoreTriangulation([1,3,1,4,1,5])).toBe(13);
  expect(minScoreTriangulation([38,76,69,32,24,35,82,30,86,77,92,3,35,20,84,67,23,58,94,10])).toBe(153657);
})