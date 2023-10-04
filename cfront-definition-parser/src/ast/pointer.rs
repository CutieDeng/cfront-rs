#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pointer<'a> {
    a: &'a !,
}