
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

 function reverseBetween(head: ListNode | null, left: number, right: number): ListNode | null {
    if (left < right) {
        const fakeHead = new ListNode(0, head);
        let sNode = fakeHead;
        for (let i = 1; i < left; i++) {
            sNode = sNode.next;
        }
        let curNode = sNode.next;
        let nextNode = curNode.next;
        for (let i = left; i < right; i++) {
            const temp = nextNode.next;
            nextNode.next = curNode;
            curNode = nextNode;
            nextNode = temp;
        }
        
        sNode.next.next = nextNode;
        sNode.next = curNode;
        return fakeHead.next;
    }
    return head;
};
