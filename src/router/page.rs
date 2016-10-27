use std::ops::Range;

pub enum Pagination {
    Offset(Range<usize>)
}
