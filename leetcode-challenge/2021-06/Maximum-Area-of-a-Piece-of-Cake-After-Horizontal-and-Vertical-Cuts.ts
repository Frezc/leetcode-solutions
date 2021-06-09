
// find max
function maxArea(h: number, w: number, horizontalCuts: number[], verticalCuts: number[]): number {
    horizontalCuts.sort((a, b) => a - b);
    verticalCuts.sort((a, b) => a - b);
    const wm = horizontalCuts.concat(h).reduce((max, cur, index) => {
        return Math.max(max, cur - horizontalCuts[index - 1]);
    });
    const vm = verticalCuts.concat(w).reduce((max, cur, index) => {
        return Math.max(max, cur - verticalCuts[index - 1]);
    });
    return (wm * vm) % (10**9 + 7);
};
