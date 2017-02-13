/*
This example shows how to integrate the `HistoryTree` with application data.
*/

extern crate history_tree;

use history_tree::HistoryTree;

fn main() {
    let mut app = App::new();
    let root = app.root();
    let mut assets = app.add("asssets".into(), root);
    let _syntax = app.add("syntax".into(), assets);
    app.print(assets, 0);

    println!("---- change ----");
    app.change("assets".into(), &mut assets);
    app.print(assets, 0);

    println!("---- undo ----");
    app.undo();
    let assets = app.children(root)[0];
    app.print(assets, 0);

    println!("---- add ----");
    let _hello = app.add("hello".into(), assets);
    let assets = app.children(root)[0];
    app.print(assets, 0);
}

/// Stores application data.
pub struct App {
    ht: HistoryTree,
    text: Vec<String>,
}

impl App {
    /// Creates a new `App`.
    pub fn new() -> App {
        App {
            ht: HistoryTree::new(),
            // Add dummy root to align indices.
            text: vec!["root".into()],
        }
    }

    /// Gets the root.
    pub fn root(&self) -> usize {self.ht.root()}

    /// Adds a node.
    pub fn add(&mut self, text: String, parent: usize) -> usize {
        let cursor = self.ht.cursor();
        self.text.truncate(cursor + 1);

        self.text.push(text);
        self.ht.add(parent)
    }

    /// Changes a node.
    pub fn change(&mut self, text: String, node: &mut usize) {
        let cursor = self.ht.cursor();
        self.text.truncate(cursor + 1);

        self.text.push(text);
        self.ht.change(node);
    }

    /// Deletes a node.
    pub fn delete(&mut self, node: usize) {
        let cursor = self.ht.cursor();
        self.text.truncate(cursor + 1);

        self.ht.delete(node);
    }

    /// Prints out data to standard output.
    pub fn print(&self, parent: usize, tabs: u32) {
        if tabs > 0 {
            for _ in 0..tabs - 1 {print!("  ")}
            print!("|-");
        }
        println!("{}", self.text[parent]);
        for &ch in &self.ht.children(parent) {
            self.print(ch, tabs + 1);
        }
    }

    /// Goes one step back in history.
    pub fn undo(&mut self) {self.ht.undo()}

    /// Goes one step forward in history.
    pub fn redo(&mut self) {self.ht.redo()}

    /// Gets children.
    pub fn children(&self, parent: usize) -> Vec<usize> {
        self.ht.children(parent)
    }
}
