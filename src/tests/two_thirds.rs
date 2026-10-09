use crate::{Node, TwoThirdsOfLinkedList};

#[test]
fn single_node_returns_none() {
    let head = Node::new(0);

    assert_eq!(TwoThirdsOfLinkedList::get_two_thirds_node(&head), None);
}

#[test]
fn two_nodes_returns_index_zero() {
    let head = Node::with_next(0, Node::new(1));

    assert_eq!(
        TwoThirdsOfLinkedList::get_two_thirds_node(&head).map(|n| n.data),
        Some(0)
    );
}

#[test]
fn three_nodes_returns_index_one() {
    let head = Node::with_next(0, Node::with_next(1, Node::new(2)));

    assert_eq!(
        TwoThirdsOfLinkedList::get_two_thirds_node(&head).map(|n| n.data),
        Some(1)
    );
}

#[test]
fn four_nodes_returns_index_one() {
    let head = Node::with_next(0, Node::with_next(1, Node::with_next(2, Node::new(3))));

    assert_eq!(
        TwoThirdsOfLinkedList::get_two_thirds_node(&head).map(|n| n.data),
        Some(1)
    );
}

#[test]
fn five_nodes_returns_index_two() {
    let head = Node::with_next(
        0,
        Node::with_next(1, Node::with_next(2, Node::with_next(3, Node::new(4)))),
    );

    assert_eq!(
        TwoThirdsOfLinkedList::get_two_thirds_node(&head).map(|n| n.data),
        Some(2)
    );
}

#[test]
fn six_nodes_returns_index_three() {
    let head = Node::with_next(
        0,
        Node::with_next(
            1,
            Node::with_next(2, Node::with_next(3, Node::with_next(4, Node::new(5)))),
        ),
    );

    assert_eq!(
        TwoThirdsOfLinkedList::get_two_thirds_node(&head).map(|n| n.data),
        Some(3)
    );
}

#[test]
fn seven_nodes_returns_index_three() {
    let head = Node::with_next(
        0,
        Node::with_next(
            1,
            Node::with_next(
                2,
                Node::with_next(3, Node::with_next(4, Node::with_next(5, Node::new(6)))),
            ),
        ),
    );

    assert_eq!(
        TwoThirdsOfLinkedList::get_two_thirds_node(&head).map(|n| n.data),
        Some(3)
    );
}

#[test]
fn eight_nodes_returns_index_four() {
    let head = Node::with_next(
        0,
        Node::with_next(
            1,
            Node::with_next(
                2,
                Node::with_next(
                    3,
                    Node::with_next(4, Node::with_next(5, Node::with_next(6, Node::new(7)))),
                ),
            ),
        ),
    );

    assert_eq!(
        TwoThirdsOfLinkedList::get_two_thirds_node(&head).map(|n| n.data),
        Some(4)
    );
}