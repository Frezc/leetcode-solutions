/**
 * @param {number[][]} points
 * @return {boolean}
 */
var isBoomerang = function(points) {
  if (same(points[0], points[1]) || same(points[0], points[2]) || same(points[2], points[1])) {
    return false;
  }

  const l1 = (points[0][0] - points[1][0]) / (points[0][1] - points[1][1]);
  const l2 = (points[0][0] - points[2][0]) / (points[0][1] - points[2][1]);
  if (l1 === l2 || (!Number.isFinite(l1) && !Number.isFinite(l2))) {
    return false;
  }

  return true;
};

function same(p1, p2) {
  return p1[0] === p2[0] && p1[1] === p2[1];
}

it('1037', () => {
  expect(isBoomerang([[1,1],[2,3],[3,2]])).toBe(true);
  expect(isBoomerang([[1,1],[2,2],[3,3]])).toBe(false);
  expect(isBoomerang([[1,0],[0,0],[2,0]])).toBe(false);
})