
// use DFS to check every case
function makesquare(matchsticks: number[]): boolean {
    const sum = matchsticks.reduce((acc, cur) => acc + cur);
    if (sum % 4 !== 0) {
        return false;
    }
    
    const size = sum / 4;
    return checkSquare(size, [0, 0, 0, 0], matchsticks, 0);
};

function checkSquare(size: number, lens: number[], rest: number[], restIndex: number) {
    if (restIndex === rest.length) {
        return true;
    }
    
    for (let i = 0; i < lens.length; i++) {
        const lls = lens.slice();
        lls[i] += rest[restIndex];
        if (lls[i] <= size) {
            if (checkSquare(size, lls, rest, restIndex + 1)) {
                return true;
            }
        }
    }
    return false;
}
