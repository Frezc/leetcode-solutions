function longestOnes(nums: number[], k: number): number {
    let max = 0;
    let len = 0;
    let restK = k;
    
    let i = 0;
    let j = 0;
    while (i < nums.length) {
        const num = nums[i];
        if (num > 0) {
            len += 1;
            if (len > max) {
                max = len;
            }
            i++;
        } else if (restK > 0) {
            restK--;
            len += 1;
            if (len > max) {
                max = len;
            }
            i++;
        } else if (j < i) {
            len -= 1;
            if (nums[j] === 0) {
                restK++;
            }
            j++;
        } else {
            i++;
            j = i;
            len = 0;
            restK = k;
        }
    }
    
    return max;
};

test('longestOnes', () => {
    expect(longestOnes([1,1,1,0,0,0,1,1,1,1,0], 2)).toBe(6);
})
