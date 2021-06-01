function maxAreaOfIsland(grid: number[][]): number {
    const m = grid.length;
    const n = grid[0].length;
    const marked = new Array(m * n).fill(false);
    let max = 0;
    
    for (let i = 0; i < m; i++) {
        for (let j = 0; j < n; j++) {
            if (!marked[i * n + j] && grid[i][j] === 1) {
                const re = travel(grid, marked, i, j);
                if (re > max) {
                    max = re;
                }
            }
        }
    }
    
    return max;
};

function travel(grid: number[][], marked: boolean[], x: number, y: number) {
    const n = grid[0].length;
    let result = 0;
    if (grid[x][y] !== 1 || marked[x * n + y]) {
        return result;
    }
    result += 1;
    marked[x * n + y] = true;
    if (x > 0) {
        result += travel(grid, marked, x - 1, y);
    }
    if (x < grid.length - 1) {
        result += travel(grid, marked, x + 1, y);
    }
    
    if (y > 0) {
        result += travel(grid, marked, x, y - 1);
    }
    
    if (y < n - 1) {
        result += travel(grid, marked, x, y + 1);
    }
    
    return result;
}