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

/**
 * 1. Solution: solute as find start of loop list
 * 
 * 先遍历两个链表，判断最后一个元素是否相同，如果是那么两个链表是相交的。
 * 然后将这个最后的元素next连到其中一个链表的head，问题就变成和找链表环的起点一样了。
 * 这里用快慢指针来解决，可以假设环起点距离为 a，环长度为b，俩指针相遇点距离环起点x，慢指针到相遇走了d，快指针走了2d，可以列方程
 * d - a = 2d - a - kb => d = kb
 * 因为 d = a + x
 * 所以 a = kb - x
 * 所以这时候有一个指针从起点，另一个从相遇点一起以同样的速度走，那么他们最终会在环的起点相遇
 * 
 * 2. 相互遍历
 * 俩指针从A B起点同时遍历，遍历到底时从另一条链表的起点开始遍历，这样在第二次遍历到终点前相遇时，这个节点就是相交点。
 * 相互遍历的情况下如果有相交点那么俩指针的路程是一定相同的，这种方法能很快找到相交点。
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
