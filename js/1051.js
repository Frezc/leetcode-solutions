/**
 * @param {number[]} heights
 * @return {number}
 */
var heightChecker = function(heights) {
  const heightMap = new Array(101).fill(0);
  for (const h of heights) {
    heightMap[h] = (heightMap[h] || 0) + 1;
  }
  for (let i = 1; i < heightMap.length; i++) {
    heightMap[i] += heightMap[i-1];
  }

  let count = 0;
  for (let i = 0; i < heights.length; i++) {
    if (i < heightMap[heights[i]-1] || i >= heightMap[heights[i]]) {
      count++;
    }
  }
  
  return count;
};

it('test', () => {
  expect(heightChecker([1,1,4,2,1,3])).toEqual(3);
  expect(heightChecker([1,2,1,2,1,1,1,2,1])).toEqual(4);
})