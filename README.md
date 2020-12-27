# lineartree

[![Crates.io Page](https://img.shields.io/crates/v/lineartree.svg)](https://crates.io/crates/lineartree)
[![Build Status](https://api.travis-ci.com/frapa/lineartree.svg)](https://crates.io/crates/lineartree)
[![Coverage Status](https://coveralls.io/repos/github/frapa/lineartree/badge.svg?branch=main)](https://coveralls.io/github/frapa/lineartree?branch=main)
[![Docs.rs Page](https://docs.rs/lineartree/badge.svg)](https://docs.rs/lineartree)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://mit-license.org/)
[![Number of Lines of Code](https://tokei.rs/b1/github/frapa/lineartree)](https://github.com/frapa/lineartree)

A simple and easy-to-use tree data structure for rust.

This crate implements trees using a single vector to hold all nodes, hence the name.
Basically it's a `Vec<Node<T>>`, where each `Node<T>` has indices of parents and children.

On top of that, there's some convenience functions to iterate depth-first and breadth-first
across nodes, find children, and so on.

## Quick Start

### Tree creation

```rust
use lineartree::{Tree, NodeRef};


/* This builds the following tree
 *        "/"
 *       /   \
 *   etc     usr
 *          /   \
 *        bin   lib
 */
 
let mut tree = Tree::new();

// Trees usually have a root node
let fs_root = tree.root("/")?;

// Using .root() or .node() return a NodeRef object
// which can be later used to identify and manipulate
// node values.
let usr = tree.node("usr");
tree.append_child(fs_root, usr)?;

// Add multiple children at once
let bin = tree.node("bin");
let lib = tree.node("lib");
tree.append_children(usr, &[bin, lib])?;

// You can also add nodes to a parent in a single go
let etc = tree.child_node(fs_root, "etc")?;
```

### Getting, changing and removing nodes

```rust
// Get node values (this is O(1))
assert_eq!(tree.get(lib), Some(&"lib"));
assert_eq!(tree.get(lib), Some(&"lib"));
assert_eq!(tree.get_mut(lib), Some(&mut "lib"));

// Remove node, this won't resize the underlying Vec
// because otherwise node references will be invalidated.
tree.remove(etc)?;
```

### Getting number of nodes

```rust
// .len() is also O(1)
assert_eq!(tree.len(), 4);
```

### Traverse tree
    
```rust
// Here are the basic hierarchical operators
assert_eq!(tree.get_parent(usr)?, Some(fs_root));
assert_eq!(
    tree.get_children(usr)?.collect::<Vec<NodeRef>>(),
    vec![bin, lib],
);

// Iterate depth first over a node children.
// Use .depth_first() to iterate the entire tree.
for node in tree.depth_first_of(usr)? {
    // ...
}
```

## Documentation

 - [API docs](https://docs.rs/lineartree/0.1.0/lineartree)
