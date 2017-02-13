# history_tree
A persistent history tree for undo/redo

This data structure makes programming of editors easier when the editor environment
is open ended, such as editors that are hosting other editors.
It makes it possible to create game engines where scripted components
are interdependent and basis for new editor functionality.

A persistent data structure is one that stores immutable data efficiently.
This allows a programming pattern that does not rely on undoing
and redoing by mutating a data structure.
Instead, you store data in blocks that is referenced by index in the history tree.

The relations between the blocks is controlled by reading out child relations.
Data blocks can reference earlier data blocks safely.
The history tree does not need to know how these references are represented,
because the consistency is guaranteed by replicating the same state of earlier trees.

This history tree stores records that points to previous version and parent.
The tree is a function of these records plus a cursor.
The cursor determine which records are active.

When a record is pointed to by a new active record, it gets overriden.
A record is considered child of a parent when it points to the parent or any previous version.

`.add`/`.change`/`.delete` are `O(1)` operations.

`.children` is `O(N * M)` operation where `N` is number of parent versions and `M` is records.

To make `.children` fast, records are stored with only indices.
