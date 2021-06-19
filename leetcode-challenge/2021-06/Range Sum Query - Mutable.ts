// how to quick exec sumRange?
// save the sum of 0 ~ n for every numbers
// sum for left to right is sums[right] - sums[left - 1]

// This problem is not strict in time space, you can use the normal solution.

class NumArray {
    sums: number[];
    constructor(private nums: number[]) {
        this.sums = nums.reduce((result, num, i) => {
            if (i > 0) {
                result.push(num + result[i - 1])
            } else {
                result.push(num);
            }
            
            return result;
        }, [] as number[]);
    }

    update(index: number, val: number): void {
        const diff = val - this.nums[index];
        for (let i = index; i < this.sums.length; i++) {
            this.sums[i] += diff;
        }
        this.nums[index] = val;
    }

    sumRange(left: number, right: number): number {
        if (left > 0) {
            return this.sums[right] - this.sums[left - 1];
        }
        return this.sums[right];
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * var obj = new NumArray(nums)
 * obj.update(index,val)
 * var param_2 = obj.sumRange(left,right)
 */
