//! A simple and easy-to-use tree data structure for rust.
//!
//! This crate implements trees using a single vector to hold all nodes, hence the name.
//! Basically it's a `Vec<Node<T>>`, where each `Node<T>` has indices of parents and children.
//!
//! On top of that, there's some convenience functions to iterate depth-first and breadth-first
//! across nodes, find children, and so on.
//!
//! ## Quick Start
//!
//! ### Tree creation
//!
//! ```rust
//! use lineartree::{Tree, NodeRef};
//!
//! /* This builds the following tree
//!  *        "/"
//!  *       /   \
//!  *   etc     usr
//!  *          /   \
//!  *        bin   lib
//!  */
//!  
//! let mut tree = Tree::new();
//!
//! // Trees usually have a root node
//! let fs_root = tree.root("/");
//!
//! // Using .root() or .node() return a NodeRef object
//! // which can be later used to identify and manipulate
//! // node values.
//! let usr = tree.node("usr");
//! tree.append_child(fs_root, usr);
//!
//! // Add multiple children at once
//! let bin = tree.node("bin");
//! let lib = tree.node("lib");
//! tree.append_children(usr, &[bin, lib]);
//!
//! let etc = tree.node("etc");
//! tree.append_child(fs_root, etc);
//! ```
//!
//! ### Getting, changing and removing nodes
//!
//! ```rust
//! // Get node values (this is O(1))
//! assert_eq!(tree.get(lib), Some(&"lib"));
//! assert_eq!(tree.get(lib), Some(&"lib"));
//! assert_eq!(tree.get_mut(lib), Some(&mut "lib"));
//!
//! // Remove node, this won't resize the underlying Vec
//! // because otherwise node references will be invalidated.
//! tree.remove(etc);
//! ```
//!
//! ### Getting number of nodes
//!
//! ```rust
//! // .len() is also O(1)
//! assert_eq!(tree.len(), 4);
//! ```
//!
//! ### Traverse tree
//!     
//! ```rust
//! // Here are the basic hierarchical operators
//! assert_eq!(tree.get_parent(usr), Some(fs_root));
//! assert_eq!(
//!     tree.get_children(usr).unwrap().collect::<Vec<NodeRef>>(),
//!     vec![bin, lib],
//! );
//!
//! // Iterate depth first over a node children.
//! // Use .depth_first() to iterate the entire tree.
//! for node in tree.depth_first_of(usr) {
//!     // ...
//! }
//! ```

use std::error::Error;
use std::fmt;
use std::slice::Iter;

// Error
// ==================================================================
#[derive(Debug, Eq, PartialEq)]
pub struct TreeError {
    message: String,
}

impl TreeError {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for TreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Error for TreeError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

type Result<T> = std::result::Result<T, TreeError>;

// NodeRef
// ==================================================================
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NodeRef {
    id: usize,
}

// Node
// ==================================================================
#[derive(Debug, Clone)]
struct Node<T> {
    content: T,
    parent: Option<NodeRef>,
    children: Vec<NodeRef>,
}

// Tree
// ==================================================================
#[derive(Debug, Clone)]
pub struct Tree<T> {
    nodes: Vec<Option<Node<T>>>,
    root: Option<NodeRef>,
    len: usize,
}

/// Represent a tree structure.
///
/// This structure is the core of the library and will own all data
/// in the tree. All functions for creating, manipulating and removing nodes,
/// as well as add children, and perform various types of iteration
/// are methods of this struct.
impl<T> Tree<T> {
    /// Create new empty tree structure.
    ///
    /// *Returns:* [Tree] struct.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
            len: 0,
        }
    }

    /// Create a root node.
    ///
    /// There can be only one root node in a tree, and calling this function
    /// twice will result in an error. Trees without root nodes are valid,
    /// but you won't be able to use some functionality like iteration
    /// over all nodes in a tree.
    ///
    /// *Arguments:*
    /// * `content` - The item to be set as content of the root node.
    ///
    /// *Returns:* A [NodeRef] object referencing the created node.
    pub fn root(&mut self, content: T) -> Result<NodeRef> {
        if self.root.is_some() {
            return Err(TreeError::new("Another root node already exists."));
        }

        let node_ref = self.node(content);
        self.root = Some(node_ref);

        Ok(node_ref)
    }

    /// Create a node.
    ///
    /// *Arguments:*
    /// * `content` - The item to be set as content of the node.
    ///
    /// *Returns:* A [NodeRef] object referencing the created node.
    pub fn node(&mut self, content: T) -> NodeRef {
        let id = self.nodes.len();

        self.nodes.push(Some(Node {
            content,
            parent: None,
            children: Vec::new(),
        }));
        self.len += 1;

        NodeRef { id }
    }

    /// Remove a node from the tree.
    ///
    /// The removed node will not reduce the amount of memory used by the
    /// tree, nor resize the underying vector so that other node
    /// references won't be invalidated.
    ///
    /// *Arguments:*
    /// * `node_ref` - [NodeRef] object indicating which node to remove.
    ///
    /// *Returns:* A result indicating whether the node was successfully
    ///            removed. Returns an error if `node_ref` is
    ///            invalid or if it was already removed.
    pub fn remove(&mut self, node_ref: NodeRef) -> Result<()> {
        match self.nodes.get(node_ref.id) {
            None => return Err(TreeError::new("Invalid node reference.")),
            Some(node) => match node {
                None => return Err(TreeError::new("Node already removed.")),
                Some(_) => self.nodes[node_ref.id] = None,
            },
        }
        self.len -= 1;
        Ok(())
    }

    /// Get the number of nodes in the tree.
    ///
    /// This is not the same as the space used by the vector
    /// implementation if some nodes where removed.
    pub fn len(&self) -> usize {
        self.len
    }

    fn get_node(&self, node_ref: NodeRef) -> Option<&Node<T>> {
        match self.nodes.get(node_ref.id) {
            None => None,
            Some(node) => node.as_ref(),
        }
    }

    fn get_node_mut(&mut self, node_ref: NodeRef) -> Option<&mut Node<T>> {
        match self.nodes.get_mut(node_ref.id) {
            None => None,
            Some(node) => node.as_mut(),
        }
    }

    /// Get immutable reference to the node content.
    ///
    /// *Arguments:*
    /// * `node_ref` - [NodeRef] object indicating which node
    ///   content to retrieve.
    ///
    /// *Returns:* Reference to the object contained in the node
    ///            or `None` if the `node_ref` is invalid.
    pub fn get(&self, node_ref: NodeRef) -> Option<&T> {
        match self.get_node(node_ref) {
            None => None,
            Some(node) => Some(&node.content),
        }
    }

    /// Get mutable reference to the node content.
    ///
    /// *Arguments:*
    /// * `node_ref` - [NodeRef] object indicating which node
    ///   content to retrieve.
    ///
    /// *Returns:* Mutable reference to the object contained in the node
    ///            or `None` if the `node_ref` is invalid.
    pub fn get_mut(&mut self, node_ref: NodeRef) -> Option<&mut T> {
        match self.get_node_mut(node_ref) {
            None => None,
            Some(node) => Some(&mut node.content),
        }
    }

    /// Add child node to a node.
    ///
    /// *Arguments:*
    /// * `parent_ref` - [NodeRef] of the parent node.
    /// * `child_ref` - [NodeRef] of the child node.
    ///
    /// *Returns:* Result indicating whether the operations was successful.
    ///            Returns an error if one of the node references is invalid.
    pub fn append_child(&mut self, parent_ref: NodeRef, child_ref: NodeRef) -> Result<()> {
        if self.get_node_mut(parent_ref).is_none() {
            return Err(TreeError::new("Parent node does not exist."));
        }

        if self.get_node_mut(child_ref).is_none() {
            return Err(TreeError::new("Child node does not exist."));
        }

        let parent_node = self.get_node_mut(parent_ref).unwrap();
        parent_node.children.push(child_ref);

        let child_node = self.get_node_mut(child_ref).unwrap();
        child_node.parent = Some(parent_ref);

        Ok(())
    }

    /// Add children nodes to a node.
    ///
    /// *Arguments:*
    /// * `parent_ref` - [NodeRef] of of the parent node.
    /// * `children_refs` - Slice of [NodeRef] for the child nodes.
    ///
    /// *Returns:* Result indicating whether the operations was successful.
    ///            Returns an error if one of the node references is invalid.
    pub fn append_children(
        &mut self,
        parent_ref: NodeRef,
        children_refs: &[NodeRef],
    ) -> Result<()> {
        for child_ref in children_refs.iter() {
            self.append_child(parent_ref, *child_ref)?;
        }
        Ok(())
    }

    /// Get iterator returning references to a node's children.
    ///
    /// *Arguments:*
    /// * `parent_ref` - [NodeRef] of the parent node.
    ///
    /// *Returns:* Iterator returning node references to the children.
    ///            Returns error if the parent reference is invalid.
    pub fn get_children(&self, parent_ref: NodeRef) -> Result<Iter<NodeRef>> {
        match self.get_node(parent_ref) {
            None => Err(TreeError::new("Parent node does not exist.")),
            Some(parent_node) => Ok(parent_node.children.iter()),
        }
    }

    /// Get reference to the parent node.
    ///
    /// *Arguments:*
    /// * `child_ref` - [NodeRef] of the child node.
    ///
    /// *Returns:* A reference to the parent node or `None` if no
    ///            parent exists. Returns error if the parent does not exist.
    pub fn get_parent(&self, child_ref: NodeRef) -> Result<Option<NodeRef>> {
        match self.get_node(child_ref) {
            None => Err(TreeError::new("Child node does not exist.")),
            Some(child_node) => Ok(child_node.parent),
        }
    }

    /// Get an iterator traversing the node and all child nodes in
    /// depth-first order.
    ///
    /// *Arguments:*
    /// * `node_ref` - [NodeRef] of the starting node.
    /// * `include_start` - If true, iteration starts with the
    ///                     starting node instead of with the first
    ///                     thereof.
    ///
    /// *Returns:* An iterator returning the node references to the
    ///            child nodes in depth-first order. Returns error
    ///            if the start node does not exist.
    pub fn depth_first_of(
        &self,
        node_ref: NodeRef,
        include_start: bool,
    ) -> Result<DepthFirstIterator<T>> {
        let mut iterator = DepthFirstIterator::new(&self, node_ref)?;
        if !include_start {
            iterator.next();
        }
        Ok(iterator)
    }

    /// Get an iterator traversing all nodes in the tree in a
    /// depth-first order.
    ///
    /// *Arguments:*
    /// * `include_root` - If true, iteration starts with the
    ///                    root node instead of with the first
    ///                    thereof.
    ///
    /// *Returns:* An iterator returning the node references to the
    ///            nodes in depth-first order. Returns error
    ///            if no root node exist.
    pub fn depth_first(&self, include_root: bool) -> Result<DepthFirstIterator<T>> {
        match self.root {
            None => Err(TreeError::new("Cannot iterate a tree without a root node.")),
            Some(root_ref) => self.depth_first_of(root_ref, include_root),
        }
    }
}

// Iterators
// ==================================================================
#[doc(hidden)]
pub struct DepthFirstIterator<'a, T> {
    tree: &'a Tree<T>,
    current: NodeRef,
    child_iterator: Box<dyn Iterator<Item = &'a NodeRef> + 'a>,
    current_iterator: Option<Box<dyn Iterator<Item = NodeRef> + 'a>>,
    finished: bool,
}

impl<'a, T> DepthFirstIterator<'a, T> {
    fn new(tree: &'a Tree<T>, current: NodeRef) -> Result<Self> {
        Ok(Self {
            tree,
            current,
            child_iterator: Box::new(tree.get_children(current)?),
            current_iterator: None,
            finished: false,
        })
    }

    fn next_child(&mut self) -> bool {
        match self.child_iterator.next() {
            None => {
                self.finished = true;
                false
            }
            Some(next_child) => {
                let child_iterator = self.tree.depth_first_of(*next_child, true).unwrap();
                self.current_iterator = Some(Box::new(child_iterator));
                true
            }
        }
    }
}

impl<'a, T> Iterator for DepthFirstIterator<'a, T> {
    type Item = NodeRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        match &mut self.current_iterator {
            None => {
                self.next_child();
                Some(self.current)
            }
            Some(iterator) => match iterator.next() {
                None => {
                    if self.next_child() {
                        self.next()
                    } else {
                        None
                    }
                }
                Some(value) => Some(value),
            },
        }
    }
}

// Tests
// ==================================================================
#[cfg(test)]
mod tests;
