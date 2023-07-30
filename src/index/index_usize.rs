macro_rules! impl_n {
    (
        $vec:ident,
        $slice:ident,
        $n:literal
    ) => { seq!(I in 0..$n {
        unsafe impl<#(T~I,)*> IndexSoa<$slice<#(T~I,)*>> for usize {
            type Output<'a> = (#(&'a T~I,)*) where $slice<#(T~I,)*>: 'a;
            type OutputMut<'a> = (#(&'a mut T~I,)*) where $slice<#(T~I,)*>: 'a;

            #[inline]
            fn index<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Self::Output<'a> {
                if self >= slice.len {
                    slice_index_bounds_fail();
                }

                unsafe {
                    // SAFETY: self is checked to be in bounds above.
                    self.get_unchecked(slice)
                }
            }

            #[inline]
            fn index_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Self::OutputMut<'a> {
                if self >= slice.len {
                    slice_index_bounds_fail();
                }

                unsafe {
                    // SAFETY: self is checked to be in bounds above.
                    self.get_unchecked_mut(slice)
                }
            }

            #[inline]
            fn get<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Option<Self::Output<'a>> {
                if self >= slice.len {
                    return None;
                }

                unsafe {
                    // SAFETY: self is checked to be in bounds above.
                    Some(self.get_unchecked(slice))
                }
            }

            #[inline]
            fn get_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Option<Self::OutputMut<'a>> {
                if self >= slice.len {
                    return None;
                }

                unsafe {
                    // SAFETY: self is checked to be in bounds above.
                    Some(self.get_unchecked_mut(slice))
                }
            }

            #[inline]
            unsafe fn get_unchecked<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Self::Output<'a> {
                debug_assert!(self < slice.len);

                unsafe {
                    // SAFETY: The caller guarantees that self is within bounds.
                    (#(&*slice.data.I.data_at(self),)*)
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Self::OutputMut<'a> {
                debug_assert!(self < slice.len);

                unsafe {
                    // SAFETY: The caller guarantees that self is within bounds.
                    (#(&mut *slice.data.I.data_mut_at(self),)*)
                }
            }
        }
    });}
}

pub(super) use impl_n;
