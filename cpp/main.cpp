#include <iostream>

class ListNode
{

public:
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution
{
public:
  ListNode *reverseList(ListNode *head)
  {
    std::cout << "hello";
    return nullptr;
  }
};

int main(int argc, char **argv)
{
  auto solver = Solution();

  ListNode *list1 = new ListNode(1);
  list1->next = new ListNode(2);
  list1->next->next = new ListNode(3);
  list1->next->next->next = new ListNode(4);
  list1->next->next->next->next = new ListNode(5);

  auto reversed_list = solver.reverseList(list1);
  std::cout << reversed_list;
}
