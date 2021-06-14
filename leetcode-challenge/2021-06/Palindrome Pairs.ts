// brute force

function palindromePairs(words: string[]): number[][] {
    const result: [number, number][] = [];
    for (let i = 0; i < words.length; i++) {
        for (let j = 0; j < words.length; j++) {
            if (i !== j && isPalindrome(words[i] + words[j])) {
                result.push([i, j]);
            }
        }
    }
    return result;
};

function isPalindrome(str: string) {
    for (let i = 0, j = str.length - 1; i <= j; i++, j--) {
        if (str[i] !== str[j]) {
            return false;
        }
    }
    
    return true;
}
