use crate::TwoThirdsOfLinkedList;

#[test]
fn empty_list_returns_none() {
    let list: TwoThirdsOfLinkedList<i32> = TwoThirdsOfLinkedList::new();

    assert_eq!(list.get_two_thirds_node(), None);
}

#[test]
fn single_node_returns_none() {
    let mut list = TwoThirdsOfLinkedList::new();
    list.prepend(0);

    assert_eq!(list.get_two_thirds_node(), None);
}

#[test]
fn two_nodes_returns_index_zero() {
    let mut list = TwoThirdsOfLinkedList::new();
    list.prepend(1).prepend(0);

    assert_eq!(list.get_two_thirds_node().map(|n| n.data), Some(0));
}

#[test]
fn task_example_three_nodes_returns_one() {
    let mut list = TwoThirdsOfLinkedList::new();
    list.prepend_iter((0..=2).rev());

    assert_eq!(list.get_two_thirds_node().map(|n| n.data), Some(1));
}

#[test]
fn task_example_four_nodes_returns_one() {
    let mut list = TwoThirdsOfLinkedList::new();
    list.prepend_iter((0..=3).rev());

    assert_eq!(list.get_two_thirds_node().map(|n| n.data), Some(1));
}

#[test]
fn task_example_five_nodes_returns_two() {
    let mut list = TwoThirdsOfLinkedList::new();
    list.prepend_iter((0..=4).rev());

    assert_eq!(list.get_two_thirds_node().map(|n| n.data), Some(2));
}

#[test]
fn large_list_one_hundred_nodes() {
    let mut list = TwoThirdsOfLinkedList::new();
    list.prepend_iter((0..100).rev());

    assert_eq!(list.get_two_thirds_node().map(|n| n.data), Some(65));
}

#[test]
fn comprehensive_range_sweep() {
    for n in 2..=100 {
        let mut list = TwoThirdsOfLinkedList::new();
        list.prepend_iter((0..n).rev());

        let expected_index = (2 * n) / 3 - 1;
        let actual = list.get_two_thirds_node().map(|node| node.data);

        assert_eq!(actual, Some(expected_index), "Test failed for n = {n}");
    }
}
