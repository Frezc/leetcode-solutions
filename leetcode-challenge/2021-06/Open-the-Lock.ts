
// Graph BFS + DP
function openLock(deadends: string[], target: string): number {
    const dp = new Set();
    deadends.forEach((de) => {
        dp.add(de);
    });
    
    let level = 0;
    let queue = [target.split('')];
    dp.add(target);
    
    while(queue.length > 0) {
        const levelQueue = queue;
        queue = [];
        
        for (let i = 0; i < levelQueue.length; i++) {
            const key = levelQueue[i].join('');
            if (key === '0000') {
                return level;
            }
            
            for (let j = 0; j < 4; j++) {
                const up = levelQueue[i].slice();
                up[j] = plusone(up[j]);
                const upstr = up.join('');
                if (!dp.has(upstr)) {
                    queue.push(up);
                    dp.add(upstr);
                }
                
                const down = levelQueue[i].slice();
                down[j] = minusone(down[j]);
                const downstr = down.join('');
                if (!dp.has(downstr)) {
                    queue.push(down);
                    dp.add(downstr);
                }
            }
        }
        
        level++;
    }
    
    return -1;
};

function plusone(base: string) {
    return String((Number(base) + 1) % 10);
}

function minusone(base: string) {
    return String(Math.max((Number(base) - 1), 0));
}

test('test', () => {
    expect(openLock(['0000'], '8888')).toBe(-1);
})
