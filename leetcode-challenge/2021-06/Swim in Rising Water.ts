// check path exist for every t
// use binary search to optimize

const dirs = [
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1]
];

function swimInWater(grid: number[][]): number {
    const n = grid.length;

    let left = 0;
    let right = n * n - 1;

    let t: number;

    function checkValid(t: number, visited: Set<number>, i: number, j: number) {
        if (i < 0 || i >= n || j < 0 || j >= n || visited.has(i * n + j) || grid[i][j] > t) {
            return false;
        }

        if (i === n - 1 && j === n - 1) {
            return true;
        }
        visited.add(i * n + j);

        for (let k = 0; k < 4; k++) {
            if (checkValid(t, visited, i + dirs[k][0], j + dirs[k][1])) {
                return true;
            }
        }
        return false;
    }
    while (left <= right) {
        t = Math.floor((left + right) / 2);
        if (checkValid(t, new Set(), 0, 0)) {
            right = t - 1;
        } else {
            left = t + 1;
        }
    }

    return left;
};



test('swim in rising water', () => {
    expect(swimInWater([[0,2],[1,3]])).toBe(3);
    expect(swimInWater([[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]])).toBe(16);
    expect(swimInWater([[7,34,16,12,15,0],[10,26,4,30,1,20],[28,27,33,35,3,8],[29,9,13,14,11,32],[31,21,23,24,19,18],[22,6,17,5,2,25]])).toBe(26);
})
