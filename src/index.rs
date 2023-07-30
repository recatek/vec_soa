use std::marker::PhantomData;
use std::ops::{Bound, Range};

use seq_macro::seq;

use crate::slice::*;
use crate::vec::*;

macro_rules! declare_vec_n {
    ($vec:ident, $trait:ident, $slice:ident, $slice_mut:ident, $n:literal) => { seq!(I in 0..$n {
        pub unsafe trait $trait<#(T~I: Sized,)*> {
            type Output<'a> where #(T~I: 'a,)*;
            type OutputMut<'a> where #(T~I: 'a,)*;

            /// Get a reference to the element at this index or range. Panics if not found.
            fn index<'a>(self, soa: &'a $vec<#(T~I,)*>) -> Self::Output<'a>;
            /// Get a mutable reference to the element at this index or range. Panics if not found.
            fn index_mut<'a>(self, soa: &'a mut $vec<#(T~I,)*>) -> Self::OutputMut<'a>;

            /// Get a reference to the element at this index or range, if within bounds.
            fn get<'a>(self, soa: &'a $vec<#(T~I,)*>) -> Option<Self::Output<'a>>;
            /// Get a mutable reference to the element at this index or range, if within bounds.
            fn get_mut<'a>(self, soa: &'a mut $vec<#(T~I,)*>) -> Option<Self::OutputMut<'a>>;

            unsafe fn get_unchecked<'a>(self, soa: &'a $vec<#(T~I,)*>) -> Self::Output<'a>;
            unsafe fn get_unchecked_mut<'a>(self, soa: &'a mut $vec<#(T~I,)*>) -> Self::OutputMut<'a>;
        }

        // ----------------------------------------------------------------------------------------

        unsafe impl<#(T~I: Sized,)*> $trait<#(T~I,)*> for usize {
            type Output<'a> = (#(&'a T~I,)*) where #(T~I: 'a,)*;
            type OutputMut<'a> = (#(&'a mut T~I,)*) where #(T~I: 'a,)*;

            #[inline]
            fn index<'a>(self, soa: &'a $vec<#(T~I,)*>) -> Self::Output<'a> {
                if self >= soa.len() {
                    panic!("index out of bounds: the len is {} but the index is {}", soa.len(), self);
                } else {
                    // SAFETY: `self` is checked to be in bounds above.
                    unsafe { (<Self as $trait<#(T~I,)*>>::get_unchecked(self, soa)) }
                }
            }

            #[inline]
            fn index_mut<'a>(self, soa: &'a mut $vec<#(T~I,)*>) -> Self::OutputMut<'a> {
                if self >= soa.len() {
                    panic!("index out of bounds: the len is {} but the index is {}", soa.len(), self);
                } else {
                    // SAFETY: `self` is checked to be in bounds above.
                    unsafe { (<Self as $trait<#(T~I,)*>>::get_unchecked_mut(self, soa)) }
                }
            }

            #[inline]
            fn get<'a>(self, soa: &'a $vec<#(T~I,)*>) -> Option<Self::Output<'a>> {
                if self >= soa.len() {
                    None
                } else {
                    // SAFETY: `self` is checked to be in bounds above.
                    unsafe { Some(<Self as $trait<#(T~I,)*>>::get_unchecked(self, soa)) }
                }
            }

            #[inline]
            fn get_mut<'a>(self, soa: &'a mut $vec<#(T~I,)*>) -> Option<Self::OutputMut<'a>> {
                if self >= soa.len() {
                    None
                } else {
                    // SAFETY: `self` is checked to be in bounds above.
                    unsafe { Some(<Self as $trait<#(T~I,)*>>::get_unchecked_mut(self, soa)) }
                }
            }

            #[inline]
            unsafe fn get_unchecked<'a>(self, soa: &'a $vec<#(T~I,)*>) -> Self::Output<'a> {
                debug_assert!(self < soa.len());

                unsafe {
                    let slices = soa.slices();
                    (#(slices.I.get_unchecked(self),)*)
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut<'a>(self, soa: &'a mut $vec<#(T~I,)*>) -> Self::OutputMut<'a> {
                debug_assert!(self < soa.len());

                unsafe {
                    let slices = soa.slices_mut();
                    (#(slices.I.get_unchecked_mut(self),)*)
                }
            }
        }

        // ----------------------------------------------------------------------------------------

        unsafe impl<#(T~I: Sized,)*> $trait<#(T~I,)*> for Range<usize> {
            type Output<'a> = $slice<'a, #(T~I,)*> where #(T~I: 'a,)*;
            type OutputMut<'a> = $slice_mut<'a, #(T~I,)*>  where #(T~I: 'a,)*;

            #[inline]
            fn index<'a>(self, soa: &'a $vec<#(T~I,)*>) -> Self::Output<'a> {
                if self.start > self.end {
                    slice_index_order_fail();
                } else if self.end > soa.len() {
                    slice_end_index_len_fail();
                }
                // SAFETY: `self` is checked to be valid and in bounds above.
                unsafe { <Self as $trait<#(T~I,)*>>::get_unchecked(self, soa) }
            }

            #[inline]
            fn index_mut<'a>(self, soa: &'a mut $vec<#(T~I,)*>) -> Self::OutputMut<'a> {
                if self.start > self.end {
                    slice_index_order_fail();
                } else if self.end > soa.len() {
                    slice_end_index_len_fail();
                }
                // SAFETY: `self` is checked to be valid and in bounds above.
                unsafe { <Self as $trait<#(T~I,)*>>::get_unchecked_mut(self, soa) }
            }

            #[inline]
            fn get<'a>(self, soa: &'a $vec<#(T~I,)*>) -> Option<Self::Output<'a>> {
                if self.start > self.end || self.end > soa.len() {
                    None
                } else {
                    // SAFETY: `self` is checked to be valid and in bounds above.
                    unsafe { Some(<Self as $trait<#(T~I,)*>>::get_unchecked(self, soa)) }
                }
            }

            #[inline]
            fn get_mut<'a>(self, soa: &'a mut $vec<#(T~I,)*>) -> Option<Self::OutputMut<'a>> {
                if self.start > self.end || self.end > soa.len() {
                    None
                } else {
                    // SAFETY: `self` is checked to be valid and in bounds above.
                    unsafe { Some(<Self as $trait<#(T~I,)*>>::get_unchecked_mut(self, soa)) }
                }
            }

            #[inline]
            unsafe fn get_unchecked<'a>(self, soa: &'a $vec<#(T~I,)*>) -> Self::Output<'a> {
                debug_assert!(self.end >= self.start);
                debug_assert!(self.end <= soa.len());

                $slice {
                    len: self.end - self.start,
                    // SAFETY: The caller guarantees that the range is in bounds.
                    data: unsafe { (#(soa.data.I.offset(self.start),)*) },
                    lifetime: PhantomData,
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut<'a>(self, soa: &'a mut $vec<#(T~I,)*>) -> Self::OutputMut<'a> {
                debug_assert!(self.end >= self.start);
                debug_assert!(self.end <= soa.len());

                $slice_mut {
                    len: self.end - self.start,
                    // SAFETY: The caller guarantees that the range is in bounds.
                    data: unsafe { (#(soa.data.I.offset(self.start),)*) },
                    lifetime: PhantomData,
                }
            }
        }
    });}
}

seq!(N in 2..=12 {
    declare_vec_n!(VecSoa~N, IndexSoa~N, SliceSoa~N, SliceSoaMut~N, N);
});

const fn slice_start_index_len_fail() -> ! {
    panic!("slice start index is out of range for slice");
}

const fn slice_end_index_len_fail() -> ! {
    panic!("slice end index is out of range for slice");
}

const fn slice_index_order_fail() -> ! {
    panic!("slice index start is larger than end");
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
