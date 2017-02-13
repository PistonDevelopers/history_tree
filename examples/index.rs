/*
This example shows working with indices.
*/

extern crate history_tree;

use history_tree::HistoryTree;

fn main() {
    let mut ht = HistoryTree::new();
    let root = ht.root();
    let _assets = ht.add(root);
    let notes = ht.add(root);
    let mut bar = ht.add(notes);
    let _baz = ht.add(bar);
    ht.print(root, 0);

    ht.change(&mut bar);
    ht.print(root, 0);

    let src = ht.add(root);
    let _file = ht.add(src);
    let _foo = ht.add(src);
    ht.print(root, 0);

    ht.delete(bar);
    ht.print(root, 0);

    println!("--------- undo ----------");
    for _ in 0..ht.records.len() - 1 {
        ht.undo();
        ht.print(root, 0);
    }

    println!("--------- redo ----------");
    for _ in 0..ht.records.len() - 1 {
        ht.redo();
        ht.print(root, 0);
    }
}
