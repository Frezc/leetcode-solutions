/**
 * @param {number[][]} workers
 * @param {number[][]} bikes
 * @return {number}
 */
var assignBikes = function(workers, bikes) {
  if (workers.length === 0 || bikes.length === 0) {
    return 0;
  }

  const dp = [];
  for (let i = 0; i < workers.length; i++) {
    dp.push([]);
    for (let j = 0; j < bikes.length; j++) {
      const d = Math.abs(workers[i][0]-bikes[j][0]) + Math.abs(workers[i][1]-bikes[j][1]);
      dp[i][j] = d;
    }
  }

  
  return assignBikes2(dp, workers, 0, bikes, 0);
};

function assignBikes2(cache, workers, si, bikes, sj) {
  if (si >= workers.length || sj >= bikes.length) {
    return 0;
  }
  let min = Infinity;
  for (let i = si; i < workers.length; i++) {
    for (let j = sj; j < bikes.length; j++) {

      const d = cache[i][j];
      const all = d + assignBikes2(cache, workers, i+1, bikes, j+1);
      
      if (all < min) {
        min = all;
      }
    }
  }
  return min;
}

var assignBikes1 = function(workers, bikes) {
  if (workers.length === 0 || bikes.length === 0) {
    return 0;
  }
  let min = Infinity;
  let mi, mj;
  for (let i = 0; i < workers.length; i++) {
    for (let j = 0; j < bikes.length; j++) {
      const d = Math.abs(workers[i][0]-bikes[j][0]) + Math.abs(workers[i][1]-bikes[j][1]);
      if (d < min) {
        min = d;
        mi = i;
        mj = j;
      }
    }
  }
  workers.splice(mi, 1);
  bikes.splice(mj, 1);
  return min + assignBikes(workers, bikes);
}

it('1057', () => {
  expect(assignBikes([[0,0],[2,1]], [[1,2],[3,3]])).toEqual(6);
  expect(assignBikes([[0,0],[1,1],[2,0]], [[1,0],[2,2],[2,1]])).toEqual(4);
  expect(assignBikes([[0,0],[1,0],[2,0],[3,0],[4,0],[5,0]], [[0,999],[1,999],[2,999],[3,999],[4,999],[5,999],[6,999],[7,999],[8,999]])).toEqual(5994);
})