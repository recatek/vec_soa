use std::iter::Iterator;

use seq_macro::seq;

use crate::slice::*;

macro_rules! impl_n {
    (
        $iter:ident,
        $iter_mut:ident,
        $slice:ident,
        $slice_ref:ident,
        $slice_ref_mut:ident,
        $n:literal
    ) => { seq!(I in 0..$n {
        pub struct $iter<'a, #(T~I: Sized,)*> {
            index: usize,
            slice: $slice_ref<'a, #(T~I,)*>,
        }

        pub struct $iter_mut<'a, #(T~I: Sized,)*> {
            index: usize,
            slice: $slice_ref_mut<'a, #(T~I,)*>,
        }

        impl<#(T~I: Sized,)*> $slice<#(T~I,)*> {
            /// Shallow-copies the slice, returning a slice that points to the same data.
            #[inline(always)]
            pub fn iter(&self) -> $iter<#(T~I,)*> {
                $iter {
                    index: 0,
                    slice: self.index(..),
                }
            }

            /// Shallow-copies the slice, returning a slice that points to the same data.
            #[inline(always)]
            pub fn iter_mut(&mut self) -> $iter_mut<#(T~I,)*> {
                $iter_mut {
                    index: 0,
                    slice: self.index_mut(..),
                }
            }
        }

        impl<'a, #(T~I: Sized + 'a,)*> Iterator for $iter<'a, #(T~I,)*> {
            type Item = (#(&'a T~I,)*);

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if self.index >= self.slice.len {
                    None
                } else {
                    let items = unsafe {
                        // SAFETY: We check above that is within bounds of the slice.
                        (#(&*self.slice.data.I.data_at(self.index),)*)
                    };
                    self.index += 1;
                    Some(items)
                }
            }
        }

        impl<'a, #(T~I: Sized + 'a,)*> Iterator for $iter_mut<'a, #(T~I,)*> {
            type Item = (#(&'a mut T~I,)*);

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if self.index >= self.slice.len {
                    None
                } else {
                    let items = unsafe {
                        // SAFETY: We check above that is within bounds of the slice.
                        (#(&mut *self.slice.data.I.data_mut_at(self.index),)*)
                    };
                    self.index += 1;
                    Some(items)
                }
            }
        }
    });}
}

seq!(N in 2..=12 {
    impl_n!(IterSoa~N, IterMutSoa~N, SliceSoa~N, SliceSoaRef~N, SliceSoaRefMut~N, N);
});
