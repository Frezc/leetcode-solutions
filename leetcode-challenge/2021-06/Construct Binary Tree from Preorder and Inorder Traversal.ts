
// emm

/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

 function buildTree(preorder: number[], inorder: number[]): TreeNode | null {
    if (preorder.length === 0) {
        return null;
    }
    
    const root = new TreeNode(preorder[0]);
    
    const index = inorder.findIndex((n) => preorder[0] === n);
    const left = buildTree(preorder.slice(1, 1 + index), inorder.slice(0, index));
    const right = buildTree(preorder.slice(1 + index), inorder.slice(index + 1));
    root.left = left;
    root.right = right;
    return root;
};
