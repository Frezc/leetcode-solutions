// just recursive brute force
function isInterleave(s1: string, s2: string, s3: string): boolean {
    const checker = new Checker(s1, s2, s3);
    return checker.check();
};

class Checker {
    constructor(private s1: string, private s2: string, private s3: string) {}
    
    check() {
        if (this.s1.length + this.s2.length !== this.s3.length) {
            return false;
        }
        
        if (this.s3.length === 0) {
            return true;
        }
        
        return this.internalCheck(0, 0, 0);
    }
    
    private internalCheck(i: number, j: number, k: number) {
        if (i < this.s1.length && this.s1[i] === this.s3[k]) {
            if (this.internalCheck(i + 1, j, k + 1)) {
                return true;
            }
        }
        if (j < this.s2.length && this.s2[j] === this.s3[k]) {
            if (this.internalCheck(i, j + 1, k + 1)) {
                return true;
            }
        }
        return k === this.s3.length;
    }
}
