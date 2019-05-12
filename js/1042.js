var gardenNoAdj = function(N, paths) {
  const result = [];
  const map = [];
  for (let i = 0; i < N; i++) {
    map.push([]);
  }
  paths.forEach((p) => {
    map[Math.max(p[0], p[1]) - 1].push(Math.min(p[0], p[1]) - 1);
  });
  map.forEach((con) => {
    if (con.length === 0) {
      result.push(1);
    } else {
      for(let i = 1;;i++) {
        if (i !== result[con[0]] && i !== result[con[1]] && i !== result[con[2]]) {
          result.push(i);
          break;
        }
      }
    }
  });

  return result;
};

it('test', () => {
  expect(gardenNoAdj(3, [[1,2],[2,3],[3,1]])).toEqual([1,2,3]);
  expect(gardenNoAdj(4, [[1,2],[3,4]])).toEqual([1,2,1,2]);
  expect(gardenNoAdj(4, [[1,2],[2,3],[3,4],[4,1],[1,3],[2,4]])).toEqual([1,2,3,4]);
  expect(gardenNoAdj(1, [])).toEqual([1]);
})