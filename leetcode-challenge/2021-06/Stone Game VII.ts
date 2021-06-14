
// dp
function stoneGameVII(stones: number[]): number {
    const dp = Array.from({ length: stones.length }, () => new Array(stones.length).fill(0));

    for (let i = stones.length - 2; i >= 0; i--) {
        let sum = stones[i];
        for (let j = i + 1; j < stones.length; j++) {
            sum += stones[j];
            dp[i][j] = Math.max(sum - stones[i] - dp[i + 1][j], sum - stones[j] - dp[i][j - 1]);
        }
    }

    return dp[0][stones.length - 1];
};

test('work', () => {
    expect(stoneGameVII([721,979,690,84,742,873,31,323,819,22,928,866,118,843,169,818,908,832,852,480,763,715,875,629])).toBe(7948);
})
