// Baseline: 1M sequential inserts + 1M sequential deletes against
// std::set<int>. std::set is a red-black tree -- self-balancing binary
// tree, same algorithmic class as our AVL, one heap node per key.
// Closest direct analog to bench_avl.cpp.

#include <chrono>
#include <cstdio>
#include <set>

int main() {
    using clock = std::chrono::steady_clock;
    using ms = std::chrono::duration<double, std::milli>;
    const int N = 1'000'000;

    std::printf("C++ std::set  N=%d sequential inserts + N sequential deletes\n", N);
    for (int trial = 1; trial <= 3; ++trial) {
        std::set<int> set;

        auto t0 = clock::now();
        for (int i = 0; i < N; ++i) set.insert(i);
        auto t1 = clock::now();
        auto len_inserts = set.size();

        for (int i = 0; i < N; ++i) set.erase(i);
        auto t2 = clock::now();
        auto len_deletes = set.size();

        std::printf("  trial %d  insert=%7.1f ms  delete=%7.1f ms  len_after_inserts=%zu  len_after_deletes=%zu\n",
                    trial,
                    ms(t1 - t0).count(),
                    ms(t2 - t1).count(),
                    len_inserts, len_deletes);
    }
    return 0;
}
