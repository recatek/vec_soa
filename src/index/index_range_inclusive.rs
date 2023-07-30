macro_rules! impl_n {
    (
        $vec:ident,
        $slice:ident,
        $slice_ref:ident,
        $slice_ref_mut:ident,
        $n:literal
    ) => { seq!(I in 0..$n {
        unsafe impl<#(T~I,)*> IndexSoa<$slice<#(T~I,)*>> for RangeInclusive<usize> {
            type Output<'a> = $slice_ref<'a, #(T~I,)*> where $slice<#(T~I,)*>: 'a;
            type OutputMut<'a> = $slice_ref_mut<'a, #(T~I,)*> where $slice<#(T~I,)*>: 'a;

            #[inline]
            fn index<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Self::Output<'a> {
                if *self.end() == usize::MAX {
                    slice_end_index_overflow_fail();
                }

                (*self.start()..*self.end()+1).index(slice)
            }

            #[inline]
            fn index_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Self::OutputMut<'a> {
                if *self.end() == usize::MAX {
                    slice_end_index_overflow_fail();
                }

                (*self.start()..*self.end()+1).index_mut(slice)
            }

            #[inline]
            fn get<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Option<Self::Output<'a>> {
                (*self.start()..*self.end()+1).get(slice)
            }

            #[inline]
            fn get_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Option<Self::OutputMut<'a>> {
                (*self.start()..*self.end()+1).get_mut(slice)
            }

            #[inline]
            unsafe fn get_unchecked<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Self::Output<'a> {
                // SAFETY: The caller guarantees that self is within bounds.
                unsafe { (*self.start()..*self.end()+1).get_unchecked(slice) }
            }

            #[inline]
            unsafe fn get_unchecked_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Self::OutputMut<'a> {
                // SAFETY: The caller guarantees that self is within bounds.
                unsafe { (*self.start()..*self.end()+1).get_unchecked_mut(slice) }
            }
        }
    });}
}

pub(super) use impl_n;
