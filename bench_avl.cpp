// Benchmark: 1M sequential inserts + 1M sequential deletes on AVL tree.

#include <algorithm>
#include <chrono>
#include <cstdio>
#include <memory>
#include <utility>

struct Node {
    int value;
    int height;
    std::unique_ptr<Node> left;
    std::unique_ptr<Node> right;
    explicit Node(int v) : value(v), height(1) {}
};

static int heightOf(const std::unique_ptr<Node>& n) { return n ? n->height : 0; }
static void updateHeight(Node& n) {
    n.height = 1 + std::max(heightOf(n.left), heightOf(n.right));
}
static int balanceFactor(const Node& n) {
    return heightOf(n.left) - heightOf(n.right);
}

static std::unique_ptr<Node> rotateRight(std::unique_ptr<Node> root) {
    auto newRoot = std::move(root->left);
    root->left = std::move(newRoot->right);
    updateHeight(*root);
    newRoot->right = std::move(root);
    updateHeight(*newRoot);
    return newRoot;
}
static std::unique_ptr<Node> rotateLeft(std::unique_ptr<Node> root) {
    auto newRoot = std::move(root->right);
    root->right = std::move(newRoot->left);
    updateHeight(*root);
    newRoot->left = std::move(root);
    updateHeight(*newRoot);
    return newRoot;
}

static std::unique_ptr<Node> rebalance(std::unique_ptr<Node> node) {
    updateHeight(*node);
    int bf = balanceFactor(*node);
    if (bf > 1) {
        if (balanceFactor(*node->left) < 0)
            node->left = rotateLeft(std::move(node->left));
        return rotateRight(std::move(node));
    } else if (bf < -1) {
        if (balanceFactor(*node->right) > 0)
            node->right = rotateRight(std::move(node->right));
        return rotateLeft(std::move(node));
    }
    return node;
}

static std::unique_ptr<Node> insertNode(std::unique_ptr<Node> node, int v) {
    if (!node) return std::make_unique<Node>(v);
    if (v < node->value)       node->left  = insertNode(std::move(node->left),  v);
    else if (v > node->value)  node->right = insertNode(std::move(node->right), v);
    else                       return node;
    return rebalance(std::move(node));
}

static std::pair<std::unique_ptr<Node>, int> takeMin(std::unique_ptr<Node> node) {
    if (node->left) {
        auto [newLeft, minValue] = takeMin(std::move(node->left));
        node->left = std::move(newLeft);
        return {rebalance(std::move(node)), minValue};
    }
    int v = node->value;
    return {std::move(node->right), v};
}

static std::pair<std::unique_ptr<Node>, bool> deleteNode(std::unique_ptr<Node> node, int v) {
    if (!node) return {nullptr, false};
    if (v < node->value) {
        auto [newLeft, found] = deleteNode(std::move(node->left), v);
        node->left = std::move(newLeft);
        return {rebalance(std::move(node)), found};
    }
    if (v > node->value) {
        auto [newRight, found] = deleteNode(std::move(node->right), v);
        node->right = std::move(newRight);
        return {rebalance(std::move(node)), found};
    }
    if (!node->left && !node->right) return {nullptr, true};
    if (!node->left)  return {std::move(node->right), true};
    if (!node->right) return {std::move(node->left),  true};
    auto [newRight, succ] = takeMin(std::move(node->right));
    node->value = succ;
    node->right = std::move(newRight);
    return {rebalance(std::move(node)), true};
}

class AvlTree {
public:
    void insert(int v) { root_ = insertNode(std::move(root_), v); }
    bool remove(int v) {
        auto [newRoot, found] = deleteNode(std::move(root_), v);
        root_ = std::move(newRoot);
        return found;
    }
    int height() const { return heightOf(root_); }
private:
    std::unique_ptr<Node> root_;
};

int main() {
    using clock = std::chrono::steady_clock;
    using ms = std::chrono::duration<double, std::milli>;
    const int N = 1'000'000;

    std::printf("C++  N=%d sequential inserts + N sequential deletes\n", N);
    for (int trial = 1; trial <= 3; ++trial) {
        AvlTree tree;

        auto t0 = clock::now();
        for (int i = 0; i < N; ++i) tree.insert(i);
        auto t1 = clock::now();
        int h_inserts = tree.height();

        for (int i = 0; i < N; ++i) tree.remove(i);
        auto t2 = clock::now();
        int h_deletes = tree.height();

        std::printf("  trial %d  insert=%7.1f ms  delete=%7.1f ms  height_after_inserts=%2d  height_after_deletes=%d\n",
                    trial,
                    ms(t1 - t0).count(),
                    ms(t2 - t1).count(),
                    h_inserts, h_deletes);
    }
    return 0;
}
