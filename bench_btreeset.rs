// Baseline: 1M sequential inserts + 1M sequential deletes against
// std::collections::BTreeSet<i32>. BTreeSet is a B-tree, not a binary
// tree -- each node holds up to ~11 keys (BTREE_B = 6), so node count
// is ~N/log(N) and cache locality is dramatically better than an AVL
// tree with one i32 per heap allocation.

use std::collections::BTreeSet;
use std::time::Instant;

fn main() {
    const N: i32 = 1_000_000;
    println!("Rust BTreeSet  N={N} sequential inserts + N sequential deletes");
    for trial in 1..=3 {
        let mut set: BTreeSet<i32> = BTreeSet::new();

        let t0 = Instant::now();
        for i in 0..N {
            set.insert(i);
        }
        let t1 = Instant::now();
        let len_inserts = set.len();

        for i in 0..N {
            set.remove(&i);
        }
        let t2 = Instant::now();
        let len_deletes = set.len();

        let ins = t1.duration_since(t0).as_secs_f64() * 1000.0;
        let del = t2.duration_since(t1).as_secs_f64() * 1000.0;
        println!("  trial {trial}  insert={ins:7.1} ms  delete={del:7.1} ms  len_after_inserts={len_inserts}  len_after_deletes={len_deletes}");
    }
}
