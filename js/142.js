/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */

/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var detectCycle = function(head) {
    if (!head || !head.next) { return null }

    let slow = head.next;
    let fast = head.next.next;
    while(true) {
        if (!(slow && fast && fast.next)) {
            return null;
        }
        if (slow === fast) {
            break;
        }
        slow = slow.next;
        fast = fast.next.next;
    }

    let iter = head;
    while(iter !== slow) {
        iter = iter.next;
        slow = slow.next;
    }
    return iter;
};
