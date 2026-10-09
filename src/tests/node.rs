use crate::Node;

#[test]
fn node_new_creates_leaf_node() {
    let node = Node::new(42);

    assert_eq!(node.data, 42);
    assert!(node.next.is_none());
}

#[test]
fn node_with_next_links_two_nodes() {
    let head = Node::with_next(10, Node::new(20));

    assert_eq!(head.data, 10);
    assert_eq!(head.next.as_ref().map(|n| n.data), Some(20));
    assert!(head.next.as_ref().unwrap().next.is_none());
}

#[test]
fn node_with_next_chains_multiple_nodes() {
    let head = Node::with_next(1, Node::with_next(2, Node::new(3)));

    assert_eq!(head.data, 1);
    assert_eq!(head.next.as_ref().map(|n| n.data), Some(2));
    assert_eq!(
        head.next
            .as_ref()
            .and_then(|n| n.next.as_ref())
            .map(|n| n.data),
        Some(3)
    );
}

#[test]
fn node_works_with_non_primitive_types() {
    let head = Node::with_next(String::from("alpha"), Node::new(String::from("beta")));

    assert_eq!(head.data.as_str(), "alpha");
    assert_eq!(head.next.as_ref().map(|n| n.data.as_str()), Some("beta"));
}

#[test]
fn node_allows_in_place_mutation() {
    let mut node = Node::new(10);
    node.data = 99;
    node.next = Some(Box::new(Node::new(100)));

    assert_eq!(node.data, 99);
    assert_eq!(node.next.as_ref().map(|n| n.data), Some(100));
}

#[test]
fn node_as_deref_traversal() {
    let head = Node::with_next(1, Node::with_next(2, Node::new(3)));

    let first_next = head.next.as_deref();
    assert_eq!(first_next.map(|n| n.data), Some(2));

    let second_next = first_next.and_then(|n| n.next.as_deref());
    assert_eq!(second_next.map(|n| n.data), Some(3));

    let third_next = second_next.and_then(|n| n.next.as_deref());
    assert!(third_next.is_none());
}
