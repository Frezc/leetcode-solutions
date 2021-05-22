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

class ListNode {
    val: number;
    next: ListNode | null;
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}

 function getIntersectionNode(headA: ListNode | null, headB: ListNode | null): ListNode | null {
    if (headA && headB) {
        let la = headA;
        while(la.next) {
            la = la.next;
        }
        let lb = headB;
        while(lb.next) {
            lb = lb.next;
        }
        
        // no intersection
        if (la !== lb) {
            return null;
        }
        la.next = headB;
        
        let s = headA;
        let f = headA;
        
        do {
            s = s.next;
            f = f.next.next;
        } while(s !== f);
        
        let r = headA;
        while(r !== s) {
            r = r.next;
            s = s.next;
        }
        
        la.next = null;
        return r;
    }
    return null;
};
