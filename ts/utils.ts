export class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
  }
}
export function getBinaryTree(array: Array<number | null>) {
  const queue: Array<TreeNode> = [];
  if (array.length > 0) {
    const root = new TreeNode(array[0]!);
    queue.push(root);
    for(let i = 1; i < array.length; i+=2) {
      const node = queue.shift()!;
      if (array[i]) {
        node.left = new TreeNode(array[i]!);
      }
      queue.push(array[i] as any && node.left);
      if (array[i+1]) {
        node.right = new TreeNode(array[i+1]!);
      }
      queue.push(array[i+1] as any && node.right);
    }
    return root;
  }
  return null;
}

it('getBinaryTree', () => {
  expect(getBinaryTree([1,2,3,null,null,null,4,null,null,null,null,null,null,null,5])).toEqual({
    val: 1,
    left: {
      val: 2,
      left: null,
      right: null,
    },
    right: {
      val: 3,
      left: null,
      right: {
        val: 4,
        left: null,
        right: {
          val: 5,
          left: null,
          right: null,
        },
      }
    }
  })
})