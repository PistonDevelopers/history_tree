//! A persistent history tree for undo/redo.
//!
//! This data structure makes programming of editors easier when the editor environment
//! is open ended, such as editors that are hosting other editors.
//! It makes it possible to create game engines where scripted components
//! are interdependent and basis for new editor functionality.
//!
//! A persistent data structure is one that stores immutable data efficiently.
//! This allows a programming pattern that does not rely on undoing
//! and redoing by mutating a data structure.
//! Instead, you store data in blocks that is referenced by index in the history tree.
//!
//! The relations between the blocks is controlled by reading out child relations.
//! Data blocks can reference earlier data blocks safely.
//! The history tree does not need to know how these references are represented,
//! because the consistency is guaranteed by replicating the same state of earlier trees.
//!
//! This history tree stores records that points to previous version and parent.
//! The tree is a function of these records plus a cursor.
//! The cursor determine which records are active.
//!
//! When a record is pointed to by a new active record, it gets overriden.
//! A record is considered child of a parent when it points to the parent or any previous version.
//!
//! `.add`/`.change`/`.delete` are `O(1)` operations.
//!
//! `.children` is `O(N * M)` operation where `N` is number of parent versions and `M` is records.
//!
//! To make `.children` fast, records are stored with only indices.

#![deny(missing_docs)]

/// Stores information about a node relation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    /// Previous version.
    pub prev: usize,
    /// Parent id.
    pub parent: usize,
    /// Removes previous nodes.
    pub remove: bool,
}

/// Stores information about history tree relations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryTree {
    /// Stores records.
    pub records: Vec<Record>,
    /// History cursor.
    /// Points to an index of records where all previous changes
    /// are active, and those after are inactive.
    /// When set to `None`, it is assumed to point to the latest version.
    pub cursor: Option<usize>,
}

impl HistoryTree {
    /// Creates a new history tree.
    pub fn new() -> HistoryTree {
        HistoryTree {
            records: vec![Record {
                prev: 0, // Points back to itself.
                parent: 0, // Points back to itself.
                remove: false,
            }],
            cursor: None,
        }
    }

    /// Gets the root.
    pub fn root(&self) -> usize {0}

    /// Gets the cursor.
    pub fn cursor(&self) -> usize {
        self.cursor.unwrap_or(self.records.len() - 1)
    }

    /// Add new node.
    pub fn add(&mut self, parent: usize) -> usize {
        let cursor = self.cursor();
        self.records.truncate(cursor + 1);
        self.cursor = None;

        let n = self.records.len();
        self.records.push(Record {
            prev: n, // Points back to itself.
            parent: parent,
            remove: false,
        });
        n
    }

    /// Change node.
    pub fn change(&mut self, node: &mut usize) {
        let cursor = self.cursor();
        self.records.truncate(cursor + 1);
        self.cursor = None;

        let n = self.records.len();
        let parent = self.records[*node].parent;
        self.records.push(Record {
            prev: *node,
            parent: parent,
            remove: false,
        });
        *node = n
    }

    /// Delete node.
    pub fn delete(&mut self, node: usize) {
        let cursor = self.cursor();
        self.records.truncate(cursor + 1);
        self.cursor = None;

        let parent = self.records[node].parent;
        self.records.push(Record {
            prev: node,
            parent: parent,
            remove: true,
        });
    }

    /// Gets the names of children.
    pub fn children(&self, parent: usize) -> Vec<usize> {
        let cursor = self.cursor.unwrap_or(self.records.len() - 1);
        if cursor < parent {return vec![];}

        let nodes: Vec<usize> = self.records[1..cursor + 1].iter()
            .enumerate()
            .filter(|&(_, r)| {
                    let mut node = parent;
                    loop {
                        if r.parent == node {return true;}
                        let new_node = self.records[node].prev;
                        if new_node == node {break;}
                        node = new_node
                    }
                    false
                })
            .map(|(i, _)| i + 1)
            .collect();

        // Remove the older versions.
        let mut new: Vec<bool> = vec![true; nodes.len()];
        for i in 0..nodes.len() {
            let a = nodes[i];
            if self.records[a].remove {new[i] = false;}
            let b = self.records[a].prev;
            if b == a  {continue;}
            if let Ok(j) = nodes.binary_search(&b) {
                new[j] = false;
            }
        }
        nodes.into_iter()
            .enumerate()
            .filter(|&(i, _)| new[i])
            .map(|(_, id)| id)
            .collect()
    }

    /// Goes back one step in history.
    pub fn undo(&mut self) {
        self.cursor = if let Some(index) = self.cursor {
            if index > 0 {Some(index - 1)}
            else if self.records.len() == 0 {None}
            else {Some(0)}
        } else {
            if self.records.len() == 0 {None}
            else {Some(self.records.len() - 2)}
        };
    }

    /// Goes forward one step in history.
    pub fn redo(&mut self) {
        self.cursor = if let Some(index) = self.cursor {
            if index + 1 >= self.records.len() {None}
            else {Some(index + 1)}
        } else {
            None
        }
    }

    /// Prints relations to standard output.
    /// This is used for debugging.
    pub fn print(&self, parent: usize, tabs: u32) {
        if tabs > 0 {
            for _ in 0..tabs - 1 {print!("  ")}
            print!("|-");
        }
        println!("{}", parent);
        for &ch in &self.children(parent) {
            self.print(ch, tabs + 1);
        }
    }
}
