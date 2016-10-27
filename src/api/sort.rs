use std::cmp::Ordering;

pub trait Sort {
    fn sorts_by(field: &str) -> bool;
    fn cmp(field: &str, left: &Self, right: &Self) -> Ordering;
}

pub trait MaybeSort {
    fn sorts_by(field: &str) -> bool;
    fn cmp(field: &str, left: &Self, right: &Self) -> Ordering;
}

impl<T> MaybeSort for T {
    default fn sorts_by(_: &str) -> bool { false }
    default fn cmp(_: &str, _: &Self, _: &Self) -> Ordering {
        unreachable!()
    }
}

impl<T: Sort> MaybeSort for T {
    fn sorts_by(field: &str) -> bool {
        <T as Sort>::sorts_by(field)
    }
    fn cmp(field: &str, left: &Self, right: &Self) -> Ordering {
        <T as Sort>::cmp(field, left, right)
    }
}
