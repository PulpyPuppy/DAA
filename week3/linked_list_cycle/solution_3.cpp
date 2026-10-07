/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    bool hasCycle(ListNode *head) {
        if (head == NULL) { return false; }
        
        ListNode *a = head, *b = head;
        
        while (b->next != NULL && b->next->next != NULL) {
            a = a->next;
            b = b->next->next;
            if (a == b) { return true; }
        }

        return false;
    }
};
