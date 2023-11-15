use tutorial::add_one;

#[test]
fn test_add_one_integration() {
    assert_eq!(4, add_one(2));
}

#[test]
fn test_add_one_integration_two() {
    assert_ne!(5, add_one(2));
}
