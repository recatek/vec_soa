pub mod index_bound;
pub mod index_range;
pub mod index_range_from;
pub mod index_range_full;
pub mod index_range_inclusive;
pub mod index_range_to;
pub mod index_range_to_inclusive;
pub mod index_usize;

use std::marker::PhantomData;
use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use seq_macro::seq;

use crate::slice::*;

pub unsafe trait IndexSoa<T> {
    type Output<'a>: 'a
    where
        T: 'a;
    type OutputMut<'a>: 'a
    where
        T: 'a;

    /// Get a reference to the element at this index or range. Panics if not found.
    fn index<'a>(self, slice: &'a T) -> Self::Output<'a>;
    /// Get a mutable reference to the element at this index or range. Panics if not found.
    fn index_mut<'a>(self, slice: &'a mut T) -> Self::OutputMut<'a>;
    /// Get a reference to the element at this index or range, if within bounds.
    fn get<'a>(self, slice: &'a T) -> Option<Self::Output<'a>>;
    /// Get a mutable reference to the element at this index or range, if within bounds.
    fn get_mut<'a>(self, slice: &'a mut T) -> Option<Self::OutputMut<'a>>;

    unsafe fn get_unchecked<'a>(self, slice: &'a T) -> Self::Output<'a>;
    unsafe fn get_unchecked_mut<'a>(self, slice: &'a mut T) -> Self::OutputMut<'a>;
}

const fn slice_start_index_overflow_fail() -> ! {
    panic!("attempted to index slice from after maximum usize");
}

const fn slice_end_index_overflow_fail() -> ! {
    panic!("attempted to index slice up to maximum usize");
}

const fn slice_index_bounds_fail() -> ! {
    panic!("slice index is out of bounds");
}

const fn slice_start_index_len_fail() -> ! {
    panic!("slice start index is out of range for slice");
}

const fn slice_end_index_len_fail() -> ! {
    panic!("slice end index is out of range for slice");
}

const fn slice_index_order_fail() -> ! {
    panic!("slice index start is larger than end");
}

/// Convert pair of `ops::Bound`s into `ops::Range`.
/// Panics on overflowing indices.
fn into_slice_range(len: usize, (start, end): (Bound<usize>, Bound<usize>)) -> Range<usize> {
    let start = match start {
        Bound::Included(start) => start,
        Bound::Excluded(start) => start
            .checked_add(1)
            .unwrap_or_else(|| slice_start_index_overflow_fail()),
        Bound::Unbounded => 0,
    };

    let end = match end {
        Bound::Included(end) => end
            .checked_add(1)
            .unwrap_or_else(|| slice_end_index_overflow_fail()),
        Bound::Excluded(end) => end,
        Bound::Unbounded => len,
    };

    // Don't bother with checking `start < end` and `end <= len`
    // since these checks are handled by `Range` impls

    start..end
}

/// Convert pair of `Bound`s into `Range`.
/// Returns `None` on overflowing indices.
fn into_range(len: usize, (start, end): (Bound<usize>, Bound<usize>)) -> Option<Range<usize>> {
    let start = match start {
        Bound::Included(start) => start,
        Bound::Excluded(start) => start.checked_add(1)?,
        Bound::Unbounded => 0,
    };

    let end = match end {
        Bound::Included(end) => end.checked_add(1)?,
        Bound::Excluded(end) => end,
        Bound::Unbounded => len,
    };

    // Don't bother with checking `start < end` and `end <= len`
    // since these checks are handled by `Range` impls

    Some(start..end)
}

/// Convert pair of `Bound`s into `Range`
/// without performing any bounds checking or (in debug) overflow checking.
fn into_range_unchecked(len: usize, (start, end): (Bound<usize>, Bound<usize>)) -> Range<usize> {
    let start = match start {
        Bound::Included(i) => i,
        Bound::Excluded(i) => i + 1,
        Bound::Unbounded => 0,
    };
    let end = match end {
        Bound::Included(i) => i + 1,
        Bound::Excluded(i) => i,
        Bound::Unbounded => len,
    };

    start..end
}

seq!(N in 2..=12 {
    index_usize::impl_n!(VecSoa~N, SliceSoa~N, N);
    index_bound::impl_n!(VecSoa~N, SliceSoa~N, SliceSoaRef~N, SliceSoaRefMut~N, N);
    index_range::impl_n!(VecSoa~N, SliceSoa~N, SliceSoaRef~N, SliceSoaRefMut~N, N);
    index_range_from::impl_n!(VecSoa~N, SliceSoa~N, SliceSoaRef~N, SliceSoaRefMut~N, N);
    index_range_full::impl_n!(VecSoa~N, SliceSoa~N, SliceSoaRef~N, SliceSoaRefMut~N, N);
    index_range_inclusive::impl_n!(VecSoa~N, SliceSoa~N, SliceSoaRef~N, SliceSoaRefMut~N, N);
    index_range_to::impl_n!(VecSoa~N, SliceSoa~N, SliceSoaRef~N, SliceSoaRefMut~N, N);
    index_range_to_inclusive::impl_n!(VecSoa~N, SliceSoa~N, SliceSoaRef~N, SliceSoaRefMut~N, N);
});
