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
        std::vector<ListNode *> nodes;
        
        while (head != NULL) {
            if (std::find(nodes.begin(), nodes.end(), head) != nodes.end()) {
                return true;
            }
            nodes.push_back(head);
            head = head->next;
        }

        return false;
    }
};
