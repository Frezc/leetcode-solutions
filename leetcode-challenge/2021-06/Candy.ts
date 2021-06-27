function candy(ratings: number[]): number {
    if (ratings.length === 1) {
        return 1;
    }
    
    const result: number[] = [];
    
    function setIncreaseCount(from: number) {
        result[from] = 1;
        let left = from - 1;
        while (left >= 0 && ratings[left] > ratings[left + 1]) {
            result[left] = Math.max(result[left + 1] + 1, result[left] || 0);
            left--;
        }
        
        let right = from + 1;
        while (right < ratings.length && ratings[right] > ratings[right - 1]) {
            result[right] = Math.max(result[right - 1] + 1, result[right] || 0);
            right++;
        }
        
        return {
            left,
            right,
        }
    }
    
    for (let i = 0; i < ratings.length;) {
        if (i === 0) {
            if (ratings[i] < ratings[i + 1]) {
                const { right } = setIncreaseCount(i);
                i = right;
            } else {
                result[i] = 1;
                i++
            }
        } else if (i === ratings.length - 1) {
            if (ratings[i] < ratings[i - 1]) {
                setIncreaseCount(i);
            } else {
                result[i] = 1;
            }
            i++;
        } else {
             if (ratings[i] <= ratings[i - 1] && ratings[i] <= ratings[i + 1]) {
                 const { right } = setIncreaseCount(i);
                 i = right;
             } else {
                 result[i] = 1;
                 i++;
             }
        }
    }
    return result.reduce((acc, cur) => acc + cur);
};

test('candy', () => {
    expect(candy([1,0,2])).toBe(5);
    expect(candy([1,2,2])).toBe(4);
    expect(candy([1,3,2,2,1])).toBe(7);
    expect(candy([1,3,4,5,2])).toBe(11);
});
