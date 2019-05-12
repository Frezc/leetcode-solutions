var maxSumAfterPartitioning = function(A, K) {
  const s = new S()
  const result = s.solve(A, K);
  return result;
};

class S {
  constructor() {
    this.dp = [];
  }

  solve(A, K, i = 0) {
    if (this.dp[i]) {
      return this.dp[i];
    }
    if (i > A.length - 1) {
      return 0;
    }

    let max = 0;
    for (let j = 1; j <= K; j++) {
      if (i+j>A.length) {
        break;
      }
      let maxsub = 0;
      for (let l = i; l < i+j; l++) {
        if (A[l] > maxsub) {
          maxsub = A[l];
        }
      }

      const v = maxsub * j + this.solve(A, K, i+j);
      if (v > max) {
        max = v;
      }
    }
    this.dp[i] = max;
    return this.dp[i];
  }
}

it('test', () => {
  expect(maxSumAfterPartitioning([1,15,7,9,2,5,10], 3)).toEqual(84);
})