
// dp? or greedy? + max heap = O(nlogn)

type MyNode = { index: number; value: number };
class MaxHeap {
    tree: MyNode[] = [{ index: 0, value: 0 }];

    push(num: MyNode) {
        this.tree.push(num);
        for (let i = this.tree.length - 1; i > 1;) {
            const parentIndex = Math.floor(i / 2);
            if (this.tree[parentIndex].value < this.tree[i].value) {
                this.swap(parentIndex, i);
                i = parentIndex;
            } else {
                break;
            }
        }
    }

    take() {
        if (this.tree.length === 1) {
            return undefined;
        }
        this.swap(1, this.tree.length - 1);
        const result = this.tree.pop();
        for (let i = 1; i * 2 < this.tree.length;) {
            let maxIndex;
            if (i * 2 + 1 >= this.tree.length || this.tree[i * 2].value > this.tree[i * 2 + 1].value) {
                maxIndex = i * 2;
            } else {
                maxIndex = i * 2 + 1;
            }
            
            if (this.tree[i].value < this.tree[maxIndex].value) {
                this.swap(i, maxIndex);
                i = maxIndex;
            } else {
                break;
            }
        }
        
        return result;
    }

    top() {
        if (this.tree.length === 1) {
            return undefined;
        }
        return this.tree[1];
    }

    clear() {
        this.tree = [{ index: 0, value: 0 }];
    }

    swap(i: number, j: number) {
        const temp = this.tree[i];
        this.tree[i] = this.tree[j];
        this.tree[j] = temp;
    }
}

function maxResult(nums: number[], k: number): number {
    if (nums.length === 1) {
        return nums[0];
    }
    const heap = new MaxHeap();
    heap.push({ index: nums.length - 1, value: nums[nums.length - 1] });
    
    for (let i = nums.length - 2; i > 0; i--) {
        let kmax = heap.top();
        const max = nums[i] + kmax.value;
        if (max > kmax.value) {
            heap.clear();
        } else {
            while (kmax && kmax.index > i + k - 1) {
                heap.take();
                kmax = heap.top();
            }
        }
        
        heap.push({ index: i, value: max });
    }
    
    return nums[0] + heap.top().value;
};
