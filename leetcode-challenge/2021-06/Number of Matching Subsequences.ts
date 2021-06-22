function numMatchingSubseq(s: string, words: string[]): number {
    return words.reduce((acc, word) => {
        return acc + (checkSubseq(s, word) ? 1 : 0);
    }, 0);
};

function checkSubseq(s: string, word: string) {
    let i = 0;
    let j = 0;
    const len1 = s.length;
    const len2 = word.length;
    while (i < len1 && j < len2) {
        if (s[i] === word[j]) {
            i++;
            j++;
        } else {
            i++;
        }
    }
    return j === len2;
}
