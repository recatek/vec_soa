use seq_macro::seq;

use crate::data::RawDataPtr;

macro_rules! declare_vec_n {
    ($name:ident, $n:literal) => { seq!(I in 0..$n {
        pub struct $name<#(T~I: Sized,)*> {
            len: usize,
            capacity: usize,
            pub(crate) data: (#(RawDataPtr<T~I>,)*),
        }

        impl<#(T~I: Sized,)*> $name<#(T~I,)*> {
            #[inline]
            pub fn new() -> Self {
                Self::with_capacity(0)
            }

            #[inline]
            pub fn with_capacity(capacity: usize) -> Self {
                Self {
                    len: 0,
                    capacity,
                    data: (#(RawDataPtr::with_capacity(capacity),)*),
                }
            }

            #[inline(always)]
            pub fn len(&self) -> usize {
                self.len
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
                    #(self.data.I.write(self.len, data.I);)*
                }

                self.len += 1;
            }

            #[inline]
            pub fn reserve(&mut self, len: usize) {
                if (self.len + len) > self.capacity {
                    let new_capacity = usize::max(self.capacity * 2, self.len + len);

                    unsafe {
                        // SAFETY: We guarantee that the current data is valid up to capacity.
                        // and that new_capacity is greater than the old self.capacity value.
                        #(self.data.I.grow(self.capacity, new_capacity);)*
                    }

                    self.capacity = new_capacity;
                }
            }

            #[inline]
            pub fn clear(&mut self) {
                unsafe {
                    // SAFETY: We guarantee all of our data is valid in the range 0..self.len.
                    #(self.data.I.drop_to(self.len);)*
                    self.len = 0;
                }
            }

            #[inline]
            pub fn swap_remove(&mut self, index: usize) -> (#(T~I,)*) {
                assert!(index < self.len);

                unsafe {
                    // SAFETY: We guarantee that the data in the range 0..self.len is valid,
                    // and we check that the index is within bounds using the assert above.
                    (#(self.data.I.swap_remove(index, self.len),)*)
                }
            }

            #[inline]
            pub fn slices(&self) -> (#(&[T~I],)*) {
                unsafe {
                    // SAFETY: We guarantee that the data is valid in the range 0..self.len.
                    (#(self.data.I.slice(0, self.len),)*)
                }
            }

            #[inline]
            pub fn slices_mut(&mut self) -> (#(&mut [T~I],)*) {
                unsafe {
                    // SAFETY: We guarantee that the data is valid in the range 0..self.len.
                    (#(self.data.I.slice_mut(0, self.len),)*)
                }
            }

            // /// Creates a SOA slice for the range `start..start+len`.
            // ///
            // /// # Safety
            // ///
            // /// It is up to the caller to guarantee that the range `start..start+len` is valid.
            // #[inline]
            // fn range_unchecked<'a>(
            //     &'a self,
            //     start: usize,
            //     len: usize
            // ) -> $slice<'a, #(T~I,)*> {
            //     debug_assert!(start + len <= self.len);

            //     unsafe {
            //         $slice {
            //             len,
            //             data: (#(self.data.I.offset(start),)*),
            //             lifetime: PhantomData,
            //         }
            //     }
            // }

            // /// Creates a mutable SOA slice for the range `start..start+len`.
            // ///
            // /// # Safety
            // ///
            // /// It is up to the caller to guarantee that the range `start..start+len` is valid.
            // #[inline]
            // fn range_unchecked_mut<'a>(
            //     &'a mut self,
            //     start: usize,
            //     len: usize
            // ) -> $slice_mut<'a, #(T~I,)*> {
            //     debug_assert!(start + len <= self.len);

            //     unsafe {
            //         $slice_mut {
            //             len,
            //             data: (#(self.data.I.offset(start),)*),
            //             lifetime: PhantomData,
            //         }
            //     }
            // }
        }

    });}
}

seq!(N in 2..=12 {
    declare_vec_n!(VecSoa~N, N);
});
