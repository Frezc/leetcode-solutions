/**
 * @param {string} S
 * @return {string}
 */
var removeDuplicates = function(S) {
  let result = S.split('');
  let removed = false;
  do {
    removed = false;

    let i = 0;
    while (i < result.length - 1) {
      if (result[i] === result[i+1]) {
        result.splice(i, 2);
        removed = true;
      } else {
        i++;
      }
    }
  } while(removed);
  return result.join('');
};

it('1047', () => {
  expect(removeDuplicates("abbaca")).toEqual("ca");
  expect(removeDuplicates("")).toEqual("");
})