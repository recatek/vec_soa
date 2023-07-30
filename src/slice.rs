use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem::{self, MaybeUninit};
use std::ops::{Index, IndexMut};
use std::ptr::{self, NonNull};
use std::slice;

use seq_macro::seq;

use crate::data::RawDataPtr;
use crate::vec::*;

macro_rules! declare_vec_n {
    ($vec:ident, $slice:ident, $slice_mut:ident, $trait:ident, $n:literal) => { seq!(I in 0..$n {
        pub struct $slice<'a, #(T~I: Sized,)*> {
            pub(crate) len: usize,
            pub(crate) data: (#(RawDataPtr<T~I>,)*),
            pub(crate) lifetime: PhantomData<&'a ()>,
        }

        pub struct $slice_mut<'a, #(T~I: Sized,)*> {
            pub(crate) len: usize,
            pub(crate) data: (#(RawDataPtr<T~I>,)*),
            pub(crate) lifetime: PhantomData<&'a mut ()>,
        }

        impl<#(T~I: Sized,)*> $vec<#(T~I,)*> {
        }
    });}
}

seq!(N in 2..=12 {
    declare_vec_n!(VecSoa~N, SliceSoa~N, SliceSoaMut~N, IndexSoa~N, N);
});
