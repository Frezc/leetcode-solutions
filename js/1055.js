/**
 * @param {number[]} A
 * @return {number}
 */
var fixedPoint = function(A) {
  return binarySearch(A, 0);
};

function binarySearch(arr, startIndex) {
  if (arr.length === 0) return -1;
  const mid = Math.floor(arr.length / 2);
  if (arr[mid] === mid + startIndex) {
    return mid + startIndex;
  } else {
    const left = binarySearch(arr.slice(0, mid), startIndex);
    if (left >= 0) {
      return left;
    }
    const right = binarySearch(arr.slice(mid + 1), startIndex + mid + 1);
    if (right >= 0) {
      return right;
    }
  }
  return -1;
}

it('1055', () => {
  expect(fixedPoint([-10,-5,0,3,7])).toEqual(3);
  expect(fixedPoint([0,2,5,8,17])).toEqual(0);
  expect(fixedPoint([-10,-5,3,4,7,9])).toEqual(-1);
})