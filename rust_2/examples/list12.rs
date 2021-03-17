#[test]
fn basic_test() {
    let mut list = LinkedList::new();
    assert_eq!(list.get_size(), 0);
    assert!(list.is_empty());

    list.push(15);
    assert_eq!(list.get_size(), 1);
    assert!(!list.is_empty());

    assert_eq!(list.pop(), Some(15));
    assert!(list.is_empty());
}
