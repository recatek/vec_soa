use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use seq_macro::seq;

use crate::data::RawDataPtr;
use crate::index::IndexSoa;
use crate::vec::*;

macro_rules! impl_n {
    (
        $vec:ident,
        $slice:ident,
        $slice_ref:ident,
        $slice_ref_mut:ident,
        $n:literal
    ) => { seq!(I in 0..$n {
        pub struct $slice<#(T~I: Sized,)*> {
            pub(crate) len: usize,
            pub(crate) data: (#(RawDataPtr<T~I>,)*),
        }

        pub struct $slice_ref<'a, #(T~I: Sized,)*> {
            pub(crate) slice: $slice<#(T~I,)*>,
            pub(crate) lifetime: PhantomData<&'a $slice<#(T~I,)*>>,
        }

        pub struct $slice_ref_mut<'a, #(T~I: Sized,)*> {
            pub(crate) slice: $slice<#(T~I,)*>,
            pub(crate) lifetime: PhantomData<&'a mut $slice<#(T~I,)*>>,
        }

        impl<#(T~I: Sized,)*> $slice<#(T~I,)*> {
            #[inline(always)]
            pub fn is_empty(&self) -> bool {
                self.len == 0
            }

            #[inline(always)]
            pub fn len(&self) -> usize {
                self.len
            }

            #[inline]
            pub fn index<'a, I: IndexSoa<Self>>(
                &'a self,
                index: I,
            ) -> I::Output<'a> {
                index.index(self)
            }

            #[inline]
            pub fn index_mut<'a, I: IndexSoa<Self>>(
                &'a mut self,
                index: I,
            ) -> I::OutputMut<'a> {
                index.index_mut(self)
            }

            #[inline]
            pub fn get<'a, I: IndexSoa<Self>>(
                &'a self,
                index: I,
            ) -> Option<I::Output<'a>> {
                index.get(self)
            }

            #[inline]
            pub fn get_mut<'a, I: IndexSoa<Self>>(
                &'a mut self,
                index: I,
            ) -> Option<I::OutputMut<'a>> {
                index.get_mut(self)
            }

            #[inline]
            pub fn soa_slices(&self) -> (#(&[T~I],)*) {
                unsafe {
                    // SAFETY: We guarantee that the data is valid in the range 0..self.len.
                    (#(self.data.I.slice(0, self.len),)*)
                }
            }

            #[inline]
            pub fn soa_mut_slices(&mut self) -> (#(&mut [T~I],)*) {
                unsafe {
                    // SAFETY: We guarantee that the data is valid in the range 0..self.len.
                    (#(self.data.I.slice_mut(0, self.len),)*)
                }
            }

            #[inline]
            pub unsafe fn get_unchecked<'a, I: IndexSoa<Self>>(
                &'a self,
                index: I,
            ) -> I::Output<'a> {
                unsafe {
                    // SAFETY: The caller guarantees that the index is within bounds.
                    index.get_unchecked(self)
                }
            }

            #[inline]
            pub unsafe fn get_unchecked_mut<'a, I: IndexSoa<Self>>(
                &'a mut self,
                index: I,
            ) -> I::OutputMut<'a> {
                unsafe {
                    // SAFETY: The caller guarantees that the index is within bounds.
                    index.get_unchecked_mut(self)
                }
            }

            /// Shallow-copies the slice, returning a slice that points to the same data.
            #[inline(always)]
            fn shallow_copy(&self) -> Self {
                $slice {
                    len: self.len,
                    data: (#(self.data.I.shallow_copy(),)*),
                }
            }
        }

        impl<#(T~I: Sized,)*> $vec<#(T~I,)*> {
            #[inline]
            pub fn as_slice(&self) -> $slice_ref<#(T~I,)*> {
                $slice_ref {
                    slice: self.slice.shallow_copy(),
                    lifetime: PhantomData,
                }
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> $slice_ref_mut<#(T~I,)*> {
                $slice_ref_mut {
                    slice: self.slice.shallow_copy(),
                    lifetime: PhantomData,
                }
            }
        }

        impl<'a, #(T~I: Sized,)*> Deref for $slice_ref<'a, #(T~I,)*> {
            type Target = $slice<#(T~I,)*>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.slice
            }
        }

        impl<'a, #(T~I: Sized,)*> Deref for $slice_ref_mut<'a, #(T~I,)*> {
            type Target = $slice<#(T~I,)*>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.slice
            }
        }

        impl<'a, #(T~I: Sized,)*> DerefMut for $slice_ref_mut<'a, #(T~I,)*> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.slice
            }
        }

        impl<'a, #(T~I: Sized,)*> Deref for $vec<#(T~I,)*> {
            type Target = $slice<#(T~I,)*>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.slice
            }
        }

        impl<'a, #(T~I: Sized,)*> DerefMut for $vec<#(T~I,)*> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.slice
            }
        }
    });}
}

seq!(N in 2..=12 {
    impl_n!(VecSoa~N, SliceSoa~N, SliceSoaRef~N, SliceSoaRefMut~N, N);
});
