use seq_macro::seq;

use crate::data::RawDataPtr;
use crate::slice::*;

macro_rules! declare_vec_n {
    ($vec:ident, $slice:ident, $n:literal) => { seq!(I in 0..$n {
        pub struct $vec<#(T~I: Sized,)*> {
            pub(crate) capacity: usize,
            pub(crate) slice: $slice<#(T~I,)*>,
        }

        impl<#(T~I: Sized,)*> $vec<#(T~I,)*> {
            #[inline]
            pub fn new() -> Self {
                Self::with_capacity(0)
            }

            #[inline]
            pub fn with_capacity(capacity: usize) -> Self {
                Self {
                    capacity,
                    slice: $slice {
                        len: 0,
                        data: (#(RawDataPtr::with_capacity(capacity),)*),
                    },

                }
            }

            #[inline(always)]
            pub fn is_empty(&self) -> bool {
                self.slice.is_empty()
            }

            #[inline(always)]
            pub fn len(&self) -> usize {
                self.slice.len
            }

            #[inline(always)]
            pub fn capacity(&self) -> usize {
                self.capacity
            }

            #[inline]
            pub fn push(&mut self, data: (#(T~I,)*)) {
                self.reserve(1);

                unsafe {
                    // SAFETY: We guarantee that the data at self.len is allocated and invalid.
                    #(self.slice.data.I.write(self.slice.len, data.I);)*
                }

                self.slice.len += 1;
            }

            #[inline]
            pub fn reserve(&mut self, len: usize) {
                if (self.slice.len + len) > self.capacity {
                    let new_capacity = usize::max(self.capacity * 2, self.slice.len + len);

                    unsafe {
                        // SAFETY: We guarantee that the current data is valid up to capacity.
                        // and that new_capacity is greater than the old self.capacity value.
                        #(self.slice.data.I.grow(self.capacity, new_capacity);)*
                    }

                    self.capacity = new_capacity;
                }
            }

            #[inline]
            pub fn clear(&mut self) {
                unsafe {
                    // SAFETY: We guarantee all of our data is valid in the range 0..self.len.
                    #(self.slice.data.I.drop_to(self.slice.len);)*
                    self.slice.len = 0;
                }
            }

            #[inline]
            pub fn swap_remove(&mut self, index: usize) -> (#(T~I,)*) {
                assert!(index < self.slice.len);

                let result = unsafe {
                    // SAFETY: We guarantee that the data in the range 0..self.len is valid,
                    // and we check that the index is within bounds using the assert above.
                    (#(self.slice.data.I.swap_remove(index, self.slice.len),)*)
                };

                self.slice.len -= 1;
                result
            }
        }

        impl<#(T~I: Sized,)*> Drop for $vec<#(T~I,)*> {
            #[inline]
            fn drop(&mut self) {
                self.clear();

                unsafe {
                    // SAFETY: We guarantee that each data pointer is allocated to self.capacity.
                    #(self.slice.data.I.dealloc(self.capacity);)*
                }
            }
        }
    });}
}

seq!(N in 2..=12 {
    declare_vec_n!(VecSoa~N, SliceSoa~N, N);
});
