use rust_dsa::{Stack, stack};

#[test]
fn integration_stack_behaviour() {
    // create a stack and manipulate it
    let mut s: Stack<i32> = stack![];
    assert!(s.is_empty());
    s.push(1);
    s.push(2);
    assert_eq!(s.len(), 2);
    assert_eq!(s.top_unchecked(), &2);
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop_unchecked(), 1);
    assert!(s.is_empty());
}

