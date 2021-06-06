function createBuckets() {
    const result = [];
    for(let i = 0; i < 10; i++) {
        result.push([]);
    }
    return result
}

function baseSort(nums: number[]) {
    for (let i = 0; i < 9; i++) {
        const buckets = createBuckets();
        nums.forEach((num) => {
            buckets[Math.floor(num % 10**(i + 1) / 10**i)].push(num);
        });
        nums = buckets.reduce((res, bucket) => res.concat(bucket));
    }
    return nums;
}

function longestConsecutive(nums: number[]): number {
    if (nums.length === 0) {
        return 0;
    }
    
    const pos = [];
    const neg = [];
    nums.forEach((num) => {
        if (num >= 0) {
            pos.push(num);
        } else {
            neg.push(-num);
        }
    });
    const sortedArr = baseSort(neg).map((n) => -n).reverse().concat(baseSort(pos));
    
    let max = 1;
    let prev = sortedArr[0];
    let len = 1;
    
    for(let i = 1; i < sortedArr.length; i++) {
        if (sortedArr[i] === prev + 1) {
            len++;
            
            if (len > max) {
                max = len;
            }
        } else if (sortedArr[i] !== prev) {
            len = 1;
        }
        
        prev = sortedArr[i];
    }
    
    return max;
};
