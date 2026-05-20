use avl_tree::AvlTree;

fn main() {
    let mut tree = AvlTree::new();

    println!("Inserting 1..=7 in increasing order");
    println!("(a plain BST would degenerate into a right spine of height 7):");
    for i in 1..=7 {
        tree.insert(i);
        println!("  insert({i})  height={}  inorder={}", tree.height(), tree);
    }

    println!();
    println!("len = {}", tree.len());
    println!("find(4):       {}", tree.find(4));
    println!("find(99):      {}", tree.find(99));

    println!();
    println!("Deletions:");
    for v in [4, 1, 7, 99] {
        let ok = tree.delete(v);
        println!(
            "  delete({v:2})  removed={:<5}  len={}  height={}  inorder={}",
            ok, tree.len(), tree.height(), tree
        );
    }
}
