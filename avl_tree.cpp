// Self-balancing AVL tree of int keys.
// Mirror of avl_tree.rs using std::unique_ptr<Node> in place of Box<Node>.
//
// The mutating helpers take and return std::unique_ptr<Node> by value, which
// is the C++ analog of Rust's "take ownership, return ownership" pattern.

#include <algorithm>
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

static int heightOf(const std::unique_ptr<Node>& n) {
    return n ? n->height : 0;
}

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
        if (balanceFactor(*node->left) < 0) {
            node->left = rotateLeft(std::move(node->left));
        }
        return rotateRight(std::move(node));
    } else if (bf < -1) {
        if (balanceFactor(*node->right) > 0) {
            node->right = rotateRight(std::move(node->right));
        }
        return rotateLeft(std::move(node));
    }
    return node;
}

static std::unique_ptr<Node> insertNode(std::unique_ptr<Node> node, int value) {
    if (!node) return std::make_unique<Node>(value);
    if (value < node->value) {
        node->left = insertNode(std::move(node->left), value);
    } else if (value > node->value) {
        node->right = insertNode(std::move(node->right), value);
    } else {
        return node; // duplicate: no-op
    }
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

static std::pair<std::unique_ptr<Node>, bool> deleteNode(std::unique_ptr<Node> node, int value) {
    if (!node) return {nullptr, false};
    if (value < node->value) {
        auto [newLeft, found] = deleteNode(std::move(node->left), value);
        node->left = std::move(newLeft);
        return {rebalance(std::move(node)), found};
    }
    if (value > node->value) {
        auto [newRight, found] = deleteNode(std::move(node->right), value);
        node->right = std::move(newRight);
        return {rebalance(std::move(node)), found};
    }
    // Found
    if (!node->left && !node->right) return {nullptr, true};
    if (!node->left)  return {std::move(node->right), true};
    if (!node->right) return {std::move(node->left),  true};
    auto [newRight, succValue] = takeMin(std::move(node->right));
    node->value = succValue;
    node->right = std::move(newRight);
    return {rebalance(std::move(node)), true};
}

class AvlTree {
public:
    AvlTree() = default;
    AvlTree(const AvlTree&) = delete;
    AvlTree& operator=(const AvlTree&) = delete;

    void insert(int v) { root_ = insertNode(std::move(root_), v); }

    bool find(int v) const {
        const Node* cur = root_.get();
        while (cur) {
            if (v == cur->value) return true;
            cur = (v < cur->value) ? cur->left.get() : cur->right.get();
        }
        return false;
    }

    bool remove(int v) {
        auto [newRoot, found] = deleteNode(std::move(root_), v);
        root_ = std::move(newRoot);
        return found;
    }

    int height() const { return heightOf(root_); }

    void printInorder() const {
        std::printf("[");
        bool first = true;
        writeInorder(root_.get(), first);
        std::printf("]");
    }

private:
    static void writeInorder(const Node* n, bool& first) {
        if (!n) return;
        writeInorder(n->left.get(), first);
        std::printf("%s%d", first ? "" : ", ", n->value);
        first = false;
        writeInorder(n->right.get(), first);
    }

    std::unique_ptr<Node> root_;
};

int main() {
    AvlTree tree;
    std::printf("Inserting 1..=7 in increasing order\n");
    std::printf("(a plain BST would degenerate into a right spine of height 7):\n");
    for (int i = 1; i <= 7; ++i) {
        tree.insert(i);
        std::printf("  insert(%d)  height=%d  inorder=", i, tree.height());
        tree.printInorder();
        std::printf("\n");
    }
    std::printf("\nfind(4):       %s\n", tree.find(4)  ? "true" : "false");
    std::printf("find(99):      %s\n", tree.find(99) ? "true" : "false");
    std::printf("\nDeletions:\n");
    for (int v : {4, 1, 7, 99}) {
        bool ok = tree.remove(v);
        std::printf("  delete(%2d)  removed=%-5s  height=%d  inorder=",
                    v, ok ? "true" : "false", tree.height());
        tree.printInorder();
        std::printf("\n");
    }
    return 0;
}
