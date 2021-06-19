
// track used left & right parenthesis, and generate with DFS

function generateParenthesis(n: number): string[] {
    const result: string[] = [];
    generateWith(result, '', n, n);
    return result;
};

function generateWith(result: string[], prefix: string, l: number, r: number) {
    if (l === 0 && r === 0) {
        result.push(prefix);
        return;
    }
    
    if (l > 0) {
        generateWith(result, prefix + '(', l - 1, r);
    }
    
    if (r > l) {
        generateWith(result, prefix + ')', l, r - 1);
    }
}
