// Benchmark: 1M sequential inserts + 1M sequential deletes on AVL tree,
// implemented with RAW POINTERS (no std::unique_ptr).
//
// The hypothesis we're testing: the unique_ptr version pays an ABI penalty
// because std::pair<std::unique_ptr<Node>, bool> is non-trivial, so it gets
// returned via a hidden pointer instead of registers. With raw Node*, both
// pair<Node*, bool> and pair<Node*, int> are trivially copyable and should
// be returned in registers.
//
// Trade-off: we now own memory management. Manual `new`/`delete` and an
// explicit recursive freeAll() in the destructor.

#include <algorithm>
#include <chrono>
#include <cstdio>
#include <utility>

struct Node {
    int value;
    int height;
    Node* left  = nullptr;
    Node* right = nullptr;
    explicit Node(int v) : value(v), height(1) {}
};

static int heightOf(const Node* n) { return n ? n->height : 0; }
static void updateHeight(Node& n) {
    n.height = 1 + std::max(heightOf(n.left), heightOf(n.right));
}
static int balanceFactor(const Node& n) {
    return heightOf(n.left) - heightOf(n.right);
}

static Node* rotateRight(Node* root) {
    Node* newRoot = root->left;
    root->left = newRoot->right;
    updateHeight(*root);
    newRoot->right = root;
    updateHeight(*newRoot);
    return newRoot;
}
static Node* rotateLeft(Node* root) {
    Node* newRoot = root->right;
    root->right = newRoot->left;
    updateHeight(*root);
    newRoot->left = root;
    updateHeight(*newRoot);
    return newRoot;
}

static Node* rebalance(Node* node) {
    updateHeight(*node);
    int bf = balanceFactor(*node);
    if (bf > 1) {
        if (balanceFactor(*node->left) < 0)
            node->left = rotateLeft(node->left);
        return rotateRight(node);
    } else if (bf < -1) {
        if (balanceFactor(*node->right) > 0)
            node->right = rotateRight(node->right);
        return rotateLeft(node);
    }
    return node;
}

static Node* insertNode(Node* node, int v) {
    if (!node) return new Node(v);
    if (v < node->value)      node->left  = insertNode(node->left,  v);
    else if (v > node->value) node->right = insertNode(node->right, v);
    else                      return node;
    return rebalance(node);
}

static std::pair<Node*, int> takeMin(Node* node) {
    if (node->left) {
        auto [newLeft, minValue] = takeMin(node->left);
        node->left = newLeft;
        return {rebalance(node), minValue};
    }
    int v = node->value;
    Node* right = node->right;
    delete node;
    return {right, v};
}

static std::pair<Node*, bool> deleteNode(Node* node, int v) {
    if (!node) return {nullptr, false};
    if (v < node->value) {
        auto [newLeft, found] = deleteNode(node->left, v);
        node->left = newLeft;
        return {rebalance(node), found};
    }
    if (v > node->value) {
        auto [newRight, found] = deleteNode(node->right, v);
        node->right = newRight;
        return {rebalance(node), found};
    }
    // Found.
    if (!node->left && !node->right) { delete node; return {nullptr, true}; }
    if (!node->left)  { Node* r = node->right; delete node; return {r, true}; }
    if (!node->right) { Node* l = node->left;  delete node; return {l, true}; }
    auto [newRight, succ] = takeMin(node->right);
    node->value = succ;
    node->right = newRight;
    return {rebalance(node), true};
}

static void freeAll(Node* n) {
    if (!n) return;
    freeAll(n->left);
    freeAll(n->right);
    delete n;
}

class AvlTree {
public:
    AvlTree() = default;
    ~AvlTree() { freeAll(root_); }
    AvlTree(const AvlTree&)            = delete;
    AvlTree& operator=(const AvlTree&) = delete;

    void insert(int v) { root_ = insertNode(root_, v); }
    bool remove(int v) {
        auto [newRoot, found] = deleteNode(root_, v);
        root_ = newRoot;
        return found;
    }
    int height() const { return heightOf(root_); }

private:
    Node* root_ = nullptr;
};

int main() {
    using clock = std::chrono::steady_clock;
    using ms = std::chrono::duration<double, std::milli>;
    const int N = 1'000'000;

    std::printf("C++ raw-ptr  N=%d sequential inserts + N sequential deletes\n", N);
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
