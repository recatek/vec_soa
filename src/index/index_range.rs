macro_rules! impl_n {
    (
        $vec:ident,
        $slice:ident,
        $slice_ref:ident,
        $slice_ref_mut:ident,
        $n:literal
    ) => { seq!(I in 0..$n {
        unsafe impl<#(T~I,)*> IndexSoa<$slice<#(T~I,)*>> for Range<usize> {
            type Output<'a> = $slice_ref<'a, #(T~I,)*> where $slice<#(T~I,)*>: 'a;
            type OutputMut<'a> = $slice_ref_mut<'a, #(T~I,)*> where $slice<#(T~I,)*>: 'a;

            #[inline]
            fn index<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Self::Output<'a> {
                if self.start > self.end {
                    slice_index_order_fail();
                } else if self.end > slice.len {
                    slice_end_index_len_fail();
                }

                // SAFETY: `self` is checked to be valid and in bounds above.
                unsafe { self.get_unchecked(slice) }
            }

            #[inline]
            fn index_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Self::OutputMut<'a> {
                if self.start > self.end {
                    slice_index_order_fail();
                } else if self.end > slice.len {
                    slice_end_index_len_fail();
                }

                // SAFETY: `self` is checked to be valid and in bounds above.
                unsafe { self.get_unchecked_mut(slice) }
            }

            #[inline]
            fn get<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Option<Self::Output<'a>> {
                if self.start > self.end {
                    return None;
                } else if self.end > slice.len {
                    return None;
                }

                // SAFETY: `self` is checked to be valid and in bounds above.
                unsafe { Some(self.get_unchecked(slice)) }
            }

            #[inline]
            fn get_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Option<Self::OutputMut<'a>> {
                if self.start > self.end {
                    return None;
                } else if self.end > slice.len {
                    return None;
                }

                // SAFETY: `self` is checked to be valid and in bounds above.
                unsafe { Some(self.get_unchecked_mut(slice)) }
            }

            #[inline]
            unsafe fn get_unchecked<'a>(
                self,
                slice: &'a $slice<#(T~I,)*>,
            ) -> Self::Output<'a> {
                debug_assert!(self.end >= self.start);
                debug_assert!(self.end <= slice.len);

                // SAFETY: The caller guarantees that self is within bounds.
                $slice_ref {
                    slice: $slice {
                        len: self.end - self.start,
                        data: unsafe { (#(slice.data.I.offset(self.start),)*) },
                    },
                    lifetime: PhantomData,
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut<'a>(
                self,
                slice: &'a mut $slice<#(T~I,)*>,
            ) -> Self::OutputMut<'a> {
                debug_assert!(self.end >= self.start);
                debug_assert!(self.end <= slice.len);

                // SAFETY: The caller guarantees that self is within bounds.
                $slice_ref_mut {
                    slice: $slice {
                        len: self.end - self.start,
                        data: unsafe { (#(slice.data.I.offset(self.start),)*) },
                    },
                    lifetime: PhantomData,
                }
            }
        }
    });}
}

pub(super) use impl_n;
