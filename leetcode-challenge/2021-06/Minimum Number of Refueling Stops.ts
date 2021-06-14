// greedy with a sorted array (or max heap)

function minRefuelStops(target: number, startFuel: number, stations: number[][]): number {
    let curPos = startFuel;
    const backupFuels = [];
    let passedStations = 0;
    
    if (curPos >= target) {
        return 0;
    }
    
    while(true) {
        while (passedStations < stations.length && stations[passedStations][0] <= curPos) {
            insert(backupFuels, stations[passedStations][1]);
            passedStations++;
        }

        if (backupFuels.length <= 0) {
            return -1;
        } else {
            curPos += backupFuels.pop();
        }
        
        if (curPos >= target) {
            return passedStations - backupFuels.length;
        }
    }
};

function insert(nums: number[], num: number) {
    nums.push(num);
    for (let i = nums.length - 1; i > 0; i--) {
        if (nums[i] < nums[i - 1]) {
            const temp = nums[i];
            nums[i] = nums[i - 1];
            nums[i - 1] = temp;
        }
    }
}
