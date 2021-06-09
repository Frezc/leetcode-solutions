
// https://frezc.github.io/2021/06/06/maximum-performance-of-a-team/
class MinHeap {
    tree: number[] = [0];

    push(n: number) {
        let index = this.tree.length;
        this.tree.push(n);
        let parent = Math.floor(index / 2);
        while (parent > 0) {
            if (this.tree[parent] > n) {
                this.swap(parent, index);
                
                index = parent;
                parent = Math.floor(index / 2);
            } else {
                break;
            }
        }
    }

    take() {
        if (this.tree.length <= 1) {
            throw new Error();
        }
        const result = this.tree[1];
        this.tree[1] = this.tree[this.tree.length - 1];
        this.tree.pop();
        for (let i = 1; i * 2 < this.tree.length;) {
            let minIndex;
            if (i * 2 + 1 >= this.tree.length || this.tree[i * 2] < this.tree[i * 2 + 1]) {
                minIndex = i * 2;
            } else {
                minIndex = i * 2 + 1;
            }
            
            if (this.tree[i] > this.tree[minIndex]) {
                this.swap(i, minIndex);
                i = minIndex;
            } else {
                break;
            }  
        }
        return result;
    }

    swap(i1: number, i2: number) {
        const temp = this.tree[i1];
        this.tree[i1] = this.tree[i2];
        this.tree[i2] = temp;
    }
}

function maxPerformance(n: number, speed: number[], efficiency: number[], k: number): number {
    const orders = efficiency.map((_, i) => i);
    orders.sort((a, b) => {
        return efficiency[b] - efficiency[a];
    });
    
    let result = 0n;
    const speedSorted = new MinHeap();
    let totalSpeed = 0n;

    orders.forEach((order, index) => {
        const sp = speed[order];
        const ef = efficiency[order];
        speedSorted.push(sp);
        
        if (index + 1 <= k) {
            totalSpeed += BigInt(sp);
        } else {
            totalSpeed += BigInt(sp - speedSorted.take());
        }
        
        const perf = totalSpeed * BigInt(ef);
        if (perf > result) {
            result = perf;
        }
    });
    
    return Number(result % 1000000007n);
};
