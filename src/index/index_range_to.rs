macro_rules! impl_n {
    (
        $vec:ident,
        $slice:ident,
        $slice_ref:ident,
        $slice_ref_mut:ident,
        $n:literal
    ) => { seq!(I in 0..$n {
        unsafe impl<#(T~I,)*> IndexSoa<$slice<#(T~I,)*>> for RangeTo<usize> {
            type Output<'a> = $slice_ref<'a, #(T~I,)*> where $slice<#(T~I,)*>: 'a;
            type OutputMut<'a> = $slice_ref_mut<'a, #(T~I,)*> where $slice<#(T~I,)*>: 'a;

            #[inline]
            fn index<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Self::Output<'a> {
                (0..self.end).index(slice)
            }

            #[inline]
            fn index_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Self::OutputMut<'a> {
                (0..self.end).index_mut(slice)
            }

            #[inline]
            fn get<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Option<Self::Output<'a>> {
                (0..self.end).get(slice)
            }

            #[inline]
            fn get_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Option<Self::OutputMut<'a>> {
                (0..self.end).get_mut(slice)
            }

            #[inline]
            unsafe fn get_unchecked<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Self::Output<'a> {
                // SAFETY: The caller guarantees that self is within bounds.
                unsafe { (0..self.end).get_unchecked(slice) }
            }

            #[inline]
            unsafe fn get_unchecked_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Self::OutputMut<'a> {
                // SAFETY: The caller guarantees that self is within bounds.
                unsafe { (0..self.end).get_unchecked_mut(slice) }
            }
        }
    });}
}

pub(super) use impl_n;
