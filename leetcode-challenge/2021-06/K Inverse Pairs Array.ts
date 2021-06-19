// see thoughts/629

function kInversePairs(n: number, k: number): number {
    const dp = Array.from({ length: n }, () => new Array(k+1).fill(0n));
    
    dp[0][0] = 1n;
    for (let i = 1; i < n; i++) {
        dp[i][0] = 1n;
        for (let j = 1; j <= k; j++) {
            dp[i][j] = dp[i-1][j] + dp[i][j-1];
            if (j > i) {
                dp[i][j] -= dp[i-1][j-i-1];
            }
        }
    }
    
    return Number(dp[n-1][k] % 1000000007n);
};

test('k inverse pairs array', () => {
    expect(kInversePairs(3, 1)).toBe(2);
    expect(kInversePairs(3, 2)).toBe(2);
    expect(kInversePairs(3, 3)).toBe(1);
    expect(kInversePairs(3, 4)).toBe(0);
    expect(kInversePairs(4, 1)).toBe(3);
    expect(kInversePairs(4, 2)).toBe(5);
    expect(kInversePairs(4, 3)).toBe(6);
    expect(kInversePairs(4, 4)).toBe(5);
    expect(kInversePairs(1000, 1000)).toBe(663677020);
});
