/**
 * @param {number[]} barcodes
 * @return {number[]}
 */
var rearrangeBarcodes = function(barcodes) {
  const count = [];
  for (const barcode of barcodes) {
    count[barcode] = (count[barcode] || 0) + 1;
  }
  const sortedNums = Object.keys(count).map((n)=>parseInt(n)).sort((a,b) =>-(count[a]||0)+(count[b]||0));

  const result = [];
  for(let i = 0;i<barcodes.length;i++) {
    for (let j = 0;j<sortedNums.length;j++) {
      if ((!result[i-1] || result[i-1] !== sortedNums[j])) {
        result.push(sortedNums[j]);
        count[sortedNums[j]]--;
        for (let k = j; k<sortedNums.length-1;k++) {
          if ((count[sortedNums[k]]||0) < (count[sortedNums[k+1]]||0)) {
            const temp = sortedNums[k];
            sortedNums[k]=sortedNums[k+1];
            sortedNums[k+1]=temp;
          } else {
            break;
          }
        }
        break;
      }
    }
  }

  if (count[sortedNums[0]] > 0) {
    result.unshift(sortedNums[0]);
  }
  return result;
};

it('test', () => {
  // expect(rearrangeBarcodes([1,1,1,2,2,2])).toEqual([2,1,2,1,2,1]);
  // expect(rearrangeBarcodes([2,2,2,1,5])).toEqual([2,5,2,1,2]);
  expect(rearrangeBarcodes([3,7,3,7,7,7,7,2,2,2])).toEqual([/*7,2,7,2,7,2,7,3,7,3*/]);
})