// for every valid number (<= right), save the subarray count that end of.
// e.g.
// [2,1,3,1]
// 2 => [2], startIndex 0
// 1 => [2, 1], same as previous
// 3 => [2, 1, 3] [1, 3] [3], currentIndex - startIndex + 1 = 3
// 1 => [2, 1, 3, 1] [1, 3, 1] [3, 1], same as previous

function numSubarrayBoundedMax(nums: number[], left: number, right: number): number {
    let result = 0;
    
    let i = 0;
    let j = 1;
    
    while(i < nums.length) {
        if (nums[i] <= right) {
            j = i + 1;
            let sum = 0;
            if (nums[i] >= left) {
                result += 1;
                sum = 1;
            }
            while (j < nums.length) {
                if (nums[j] >= left && nums[j] <= right) {
                    sum = 1 + j - i;
                } else if (nums[j] > right) {
                    break;
                }
                result += sum;
                j++;
            }
            
            i = j + 1;
        } else {
            i++;
        }
    }

    return result;
};
