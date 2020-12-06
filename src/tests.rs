use super::*;

#[derive(Debug, Eq, PartialEq, Clone)]
struct TestData {
    field: i32,
}

fn tree2() -> (Tree<&'static str>, NodeRef, NodeRef) {
    let mut tree = Tree::new();
    let node_a = tree.node("Node A");
    let node_b = tree.node("Node B");
    (tree, node_a, node_b)
}

fn tree3() -> (Tree<&'static str>, NodeRef, NodeRef, NodeRef) {
    let mut tree = Tree::new();
    let node_a = tree.node("Node A");
    let node_b = tree.node("Node B");
    let node_c = tree.node("Node C");
    (tree, node_a, node_b, node_c)
}

fn nested_tree() -> (Tree<TestData>, NodeRef) {
    /*
     *           A
     *           |
     *    B -----+----- C
     *    |             |
     *    D         E --+-- F
     */

    let mut tree = Tree::new();

    let node_a = tree.root(TestData { field: 1 }).unwrap();
    let node_b = tree.node(TestData { field: 2 });
    let node_c = tree.node(TestData { field: 3 });
    let node_d = tree.node(TestData { field: 4 });
    let node_e = tree.node(TestData { field: 5 });
    let node_f = tree.node(TestData { field: 6 });

    tree.append_children(node_a, &[node_b, node_c]).unwrap();
    tree.append_child(node_b, node_d).unwrap();
    tree.append_children(node_c, &[node_e, node_f]).unwrap();

    (tree, node_c)
}

#[test]
fn new_node() {
    let (tree, node_a, node_b) = tree2();

    assert_eq!(tree.len(), 2);
    assert_eq!(tree.get(node_a), Some(&"Node A"));
    assert_eq!(tree.get(node_b), Some(&"Node B"));
}

#[test]
fn remove_node() {
    let (mut tree, node_a, node_b) = tree2();

    assert_eq!(tree.remove(node_a), Ok(()));

    assert_eq!(tree.len(), 1);
    assert_eq!(tree.get(node_a), None);
    assert_eq!(tree.get(node_b), Some(&"Node B"));
}

#[test]
fn remove_node_error_already_removed() {
    let (mut tree, node_a, _) = tree2();

    assert_eq!(tree.remove(node_a), Ok(()));
    assert_eq!(
        tree.remove(node_a),
        Err(TreeError::new("Node already removed."))
    );
}

#[test]
fn remove_node_error_invalid_ref() {
    let (mut tree1, _, _) = tree2();
    let (_, _, _, node2) = tree3();

    assert_eq!(
        tree1.remove(node2),
        Err(TreeError::new("Invalid node reference."))
    );
}

#[test]
fn get_mut() {
    let mut tree = Tree::new();
    let node = tree.node(TestData { field: 3 });

    tree.get_mut(node).unwrap().field = 4;

    assert_eq!(tree.get(node), Some(&TestData { field: 4 }));
}

#[test]
fn append_child() {
    let (mut tree, node_a, node_b, node_c) = tree3();

    tree.append_child(node_a, node_b).unwrap();
    tree.append_child(node_a, node_c).unwrap();

    let mut children = tree.get_children(node_a).unwrap();
    assert_eq!(*children.next().unwrap(), node_b);
    assert_eq!(*children.next().unwrap(), node_c);
    assert_eq!(children.next(), None);
}

#[test]
fn append_children() {
    let (mut tree, node_a, node_b, node_c) = tree3();

    tree.append_children(node_a, &[node_b, node_c]).unwrap();

    let mut children = tree.get_children(node_a).unwrap();
    assert_eq!(*children.next().unwrap(), node_b);
    assert_eq!(*children.next().unwrap(), node_c);
    assert_eq!(children.next(), None);
}

#[test]
fn get_parent() {
    let (mut tree, node_a, node_b, node_c) = tree3();

    tree.append_children(node_a, &[node_b, node_c]).unwrap();

    assert_eq!(tree.get_parent(node_c), Some(node_a));
    assert_eq!(tree.get_parent(node_a), None);
}

#[test]
fn append_child_error() {
    let (mut tree1, node1, _) = tree2();
    let (_, _, _, node_c2) = tree3();

    assert_eq!(
        tree1.append_child(node_c2, node1),
        Err(TreeError::new("Parent node does not exist."))
    );
    assert_eq!(
        tree1.append_child(node1, node_c2),
        Err(TreeError::new("Child node does not exist."))
    );
}

#[test]
fn clone_tree() {
    let (tree, node) = nested_tree();
    let mut clone = tree.clone();

    clone.get_mut(node).unwrap().field = 100;

    assert_eq!(clone.len(), 6);
    assert_eq!(tree.get(node).unwrap().field, 3);
    assert_eq!(clone.get(node).unwrap().field, 100);
}
