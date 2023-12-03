use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct X;

#[test]
fn context() {
    let mut context = Context::default();
    let x = X;
    assert!(context.insert(x).is_none());
    assert!(context.get::<X>().is_some());
    assert!(context.insert(x).is_some());
}

#[test]
fn reference() {
    let x = X;
    let ref_x = Ref::new(x);
    assert_eq!(x, *ref_x);
}
