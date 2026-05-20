#include <cstdio>
#include <memory>
#include <utility>

struct Node {
    int value;
    std::unique_ptr<Node> next;
};

class LinkedList {
public:
    LinkedList() = default;
    LinkedList(const LinkedList&) = delete;
    LinkedList& operator=(const LinkedList&) = delete;

    void insert(int v) {
        auto n = std::make_unique<Node>();
        n->value = v;
        n->next = std::move(head_);
        head_ = std::move(n);
    }

    bool find(int v) const {
        const Node* cur = head_.get();
        while (cur) {
            if (cur->value == v) return true;
            cur = cur->next.get();
        }
        return false;
    }

    bool remove(int v) {
        std::unique_ptr<Node>* cur = &head_;
        while (*cur) {
            if ((*cur)->value == v) {
                *cur = std::move((*cur)->next);
                return true;
            }
            cur = &(*cur)->next;
        }
        return false;
    }

    ~LinkedList() {
        auto cur = std::move(head_);
        while (cur) {
            auto next = std::move(cur->next);
            cur = std::move(next);
        }
    }

    void print() const {
        std::printf("[");
        const Node* cur = head_.get();
        bool first = true;
        while (cur) {
            std::printf("%s%d", first ? "" : ", ", cur->value);
            first = false;
            cur = cur->next.get();
        }
        std::printf("]\n");
    }

private:
    std::unique_ptr<Node> head_;
};

int main() {
    LinkedList list;
    list.insert(3);
    list.insert(2);
    list.insert(1);
    std::printf("after inserts:  ");
    list.print();

    std::printf("find(2):        %s\n", list.find(2)    ? "true" : "false");
    std::printf("find(9):        %s\n", list.find(9)    ? "true" : "false");

    std::printf("delete(2):      %s\n", list.remove(2)  ? "true" : "false");
    std::printf("after delete:   ");
    list.print();

    std::printf("delete(1):      %s\n", list.remove(1)  ? "true" : "false");
    std::printf("after delete:   ");
    list.print();

    std::printf("delete(9):      %s\n", list.remove(9)  ? "true" : "false");
    std::printf("after delete:   ");
    list.print();
    return 0;
}
