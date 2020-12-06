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

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
            len: 0,
        }
    }

    pub fn root(&mut self, content: T) -> Result<NodeRef> {
        if self.root.is_some() {
            return Err(TreeError::new("Another root node already exists."));
        }

        let node_ref = self.node(content);
        self.root = Some(node_ref);

        Ok(node_ref)
    }

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

    pub fn get(&self, node_ref: NodeRef) -> Option<&T> {
        match self.get_node(node_ref) {
            None => None,
            Some(node) => Some(&node.content),
        }
    }

    pub fn get_mut(&mut self, node_ref: NodeRef) -> Option<&mut T> {
        match self.get_node_mut(node_ref) {
            None => None,
            Some(node) => Some(&mut node.content),
        }
    }

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

    pub fn get_children(&self, parent_ref: NodeRef) -> Result<Iter<NodeRef>> {
        match self.get_node(parent_ref) {
            None => Err(TreeError::new("Node does not exist.")),
            Some(parent_node) => Ok(parent_node.children.iter()),
        }
    }

    pub fn get_parent(&self, child_ref: NodeRef) -> Option<NodeRef> {
        match self.get_node(child_ref) {
            None => None,
            Some(child_node) => child_node.parent,
        }
    }
}

// Tests
// ==================================================================
#[cfg(test)]
mod tests;
