/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {TreeNode}
 */
var bstToGst = function(root) {
  sumBst(root);
  return root;
};

function sumBst(root, sum = 0) {
  if (root) {
    const newSum = sumBst(root.right, sum);
    root.val += newSum ? newSum : sum;
    return sumBst(root.left, root.val) || root.val;
  }
}

it('1038', () => {
  expect(bstToGst())
})