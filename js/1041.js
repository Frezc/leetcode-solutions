var isRobotBounded = function(instructions) {
  const position = [0, 0];
  let direction = 0;
  const dirs = [[0,1],[1,0],[0,-1],[-1,0]];
  for (let i = 0; i < instructions.length * 4; i++) {
    switch(instructions[i%instructions.length]) {
      case 'G':
        position[0] += dirs[direction][0];
        position[1] += dirs[direction][1];
        break;
      case 'L':
        direction = (direction - 1 + dirs.length) % dirs.length;
        break;
      case 'R':
        direction = (direction + 1) % dirs.length;
        break;
    }
  }

  return position[0] === 0 && position[1] === 0;
};

it('test', () => {
  expect(isRobotBounded('GGLLGG')).toEqual(true);
  expect(isRobotBounded('GG')).toEqual(false);
  expect(isRobotBounded('GL')).toEqual(true);
  expect(isRobotBounded('')).toEqual(true);
})