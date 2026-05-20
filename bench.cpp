#include <chrono>
#include <cstdio>
#include <memory>
#include <utility>

struct Node {
    int value;
    std::unique_ptr<Node> next;
};

class LinkedList {
public:
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

    ~LinkedList() {
        auto cur = std::move(head_);
        while (cur) {
            auto next = std::move(cur->next);
            cur = std::move(next);
        }
    }

private:
    std::unique_ptr<Node> head_;
};

int main() {
    using clock = std::chrono::steady_clock;
    using ms = std::chrono::duration<double, std::milli>;

    const int N = 1'000'000;
    const int M = 50;

    std::printf("C++  N=%d full-traversal misses M=%d\n", N, M);
    for (int trial = 1; trial <= 3; ++trial) {
        auto list = std::make_unique<LinkedList>();

        auto t0 = clock::now();
        for (int i = 0; i < N; ++i) list->insert(i);
        auto t1 = clock::now();

        volatile int hits = 0;
        for (int j = 0; j < M; ++j) if (list->find(-1)) hits++;
        auto t2 = clock::now();

        list.reset();
        auto t3 = clock::now();

        std::printf("  trial %d  build=%7.1f ms  find=%7.1f ms  drop=%7.1f ms\n",
                    trial,
                    ms(t1 - t0).count(),
                    ms(t2 - t1).count(),
                    ms(t3 - t2).count());
    }
    return 0;
}
