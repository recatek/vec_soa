use std::alloc::{self, Layout};
use std::mem::{self, MaybeUninit};
use std::ptr::{self, NonNull};
use std::slice;

pub struct RawDataPtr<T>(NonNull<MaybeUninit<T>>);

unsafe impl<T> Send for RawDataPtr<T> where T: Send {}
unsafe impl<T> Sync for RawDataPtr<T> where T: Sync {}

impl<T> RawDataPtr<T> {
    /// Allocates a new data array with the given capacity, if any.
    ///
    /// If `T` is zero-sized, or the given capacity is 0, this will not allocate.
    ///
    /// # Panics
    ///
    /// This operation will panic if there is not enough memory to perform the new
    /// allocation, or if the resulting allocation size is greater than `isize::MAX`.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        if (mem::size_of::<T>() == 0) || (capacity == 0) {
            return Self(NonNull::dangling());
        }

        let layout = new_layout::<T>(capacity);

        debug_assert!(capacity > 0);
        debug_assert!(layout.size() > 0);

        assert!(layout.size() <= isize::MAX as usize, "allocation too large");

        unsafe { Self(resolve_ptr(alloc::alloc(layout), layout)) }
    }

    /// Shallow-copies this pointer, returning a pointer to the same data in memory.
    #[inline(always)]
    pub fn shallow_copy(&self) -> Self {
        Self(self.0)
    }

    /// Returns the raw pointer for this storage, which may be dangling or uninitialized.
    #[inline(always)]
    pub fn raw_ptr(&mut self) -> NonNull<MaybeUninit<T>> {
        self.0
    }

    /// Returns a raw pointer to the data at the given index.
    ///
    /// It is up to the caller to guarantee the following:
    /// - The `index` points to valid and initialized data from this pointer.
    #[inline(always)]
    pub fn data_at(&self, index: usize) -> *const T {
        unsafe {
            // SAFETY: The caller guarantees that the index is within bounds, so the
            // pointer will point to allocated data. The caller also guarantees that
            // the data is valid at this index. We can cast a MaybeUninit<T> pointer
            // to a T pointer since they have the same representation in memory.
            self.offset(index).raw_ptr().as_ptr() as *const T
        }
    }

    /// Returns a raw mutable pointer to the data at the given index.
    ///
    /// It is up to the caller to guarantee the following:
    /// - The `index` points to valid and initialized data from this pointer.
    #[inline(always)]
    pub fn data_mut_at(&mut self, index: usize) -> *mut T {
        unsafe {
            // SAFETY: The caller guarantees that the index is within bounds, so the
            // pointer will point to allocated data. The caller also guarantees that
            // the data is valid at this index. We can cast a MaybeUninit<T> pointer
            // to a T pointer since they have the same representation in memory.
            self.offset(index).raw_ptr().as_ptr() as *mut T
        }
    }

    /// Reallocates this array's old data block into a new data block.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the following:
    /// - `capacity >= old_capacity`
    /// - This array has exactly `old_capacity` elements allocated (may be 0)
    ///
    /// # Panics
    ///
    /// This operation will panic if there is not enough memory to perform the new
    /// allocation, or if the resulting allocation size is greater than `isize::MAX`.
    #[inline]
    pub unsafe fn grow(&mut self, old_capacity: usize, capacity: usize) {
        debug_assert!(capacity >= old_capacity);

        if (mem::size_of::<T>() == 0) || (capacity == 0) {
            return; // Stay dangling
        }

        let layout = new_layout::<T>(capacity);
        let size = layout.size();
        debug_assert!(size > 0);

        assert!(layout.size() <= isize::MAX as usize, "allocation too large");

        unsafe {
            if old_capacity == 0 {
                // SAFETY: The caller guarantees that capacity > 0.
                self.0 = resolve_ptr(alloc::alloc(layout), layout);
            } else {
                // SAFETY: The caller guarantees that this is allocated.
                let old_ptr = self.0.as_ptr() as *mut u8;
                // SAFETY: We checked that T is not a ZST and old_capacity > 0.
                let old_layout = Layout::array::<T>(old_capacity).unwrap();
                debug_assert!(old_layout.size() > 0);

                // SAFETY: The caller guarantees that capacity > 0.
                self.0 = resolve_ptr(alloc::realloc(old_ptr, old_layout, size), layout);
            }
        }
    }

    /// Deallocates this array's data block.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the following:
    /// - This array has exactly `capacity` elements allocated
    #[inline]
    pub unsafe fn dealloc(&mut self, capacity: usize) {
        if (mem::size_of::<T>() == 0) || (capacity == 0) {
            return; // Nothing to deallocate
        }

        // SAFETY: We checked that T is not a ZST and capacity > 0.
        let layout = Layout::array::<T>(capacity).unwrap();
        debug_assert!(layout.size() > 0);

        unsafe {
            // SAFETY: We know that old_layout has a nonzero size
            alloc::dealloc(self.0.as_ptr() as *mut u8, layout);
            self.0 = NonNull::dangling();
        }
    }

    /// Returns a pointer to the given index as an offset from this pointer.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the following:
    /// - Both this pointer and the computed offset pointer are within bounds.
    /// - The computed offset pointer, *in bytes*, does not overflow an `isize`.
    #[inline(always)]
    pub unsafe fn offset(&self, offset: usize) -> Self {
        if mem::size_of::<T>() == 0 {
            return Self(NonNull::dangling());
        }

        unsafe {
            // SAFETY: Either this is a ZST or the caller guarantees this pointer is allocated.
            Self(NonNull::new_unchecked(self.0.as_ptr().add(offset)))
        }
    }

    /// Gets the raw stored data up to `len` as a mutable `MaybeUninit<T>` slice.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the following:
    /// - This array has at least `len` elements allocated
    #[inline(always)]
    pub unsafe fn raw_data(&mut self, len: usize) -> &mut [MaybeUninit<T>] {
        // SAFETY: The caller guarantees that we have at least len elements allocated.
        unsafe { slice::from_raw_parts_mut(self.0.as_ptr(), len) }
    }

    /// Writes an element to the given index.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the following:
    /// - This pointer has allocated at least `index` elements
    /// - The element at `index` is currently invalid
    #[inline(always)]
    pub unsafe fn write(&mut self, index: usize, val: T) {
        unsafe {
            // SAFETY: The caller guarantees that this slot is allocated and invalid.
            (*self.0.as_ptr().add(index)).write(val);
        }
    }

    /// Gets a slice for the range `start..(start+len)`.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the following:
    /// - This pointer has allocated at least `len` elements
    /// - All elements the range `start..(start+len)` are valid
    #[inline(always)]
    pub unsafe fn slice(&self, start: usize, len: usize) -> &[T] {
        unsafe {
            // SAFETY: Casting a `[MaybeUninit<T>]` to a `[T]` is safe because the caller
            // guarantees that this portion of the data is valid and `MaybeUninit<T>` is
            // guaranteed to have the same layout as `T`. The pointer obtained is valid
            // since it refers to memory owned by `slice` which is a reference and thus
            // guaranteed to be valid for reads.
            // Ref: https://doc.rust-lang.org/stable/src/core/mem/maybe_uninit.rs.html#972
            slice::from_raw_parts(self.0.as_ptr().add(start) as *const T, len)
        }
    }

    /// Gets a mutable slice for the range `start..(start+len)`.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the following:
    /// - This pointer has allocated at least `len` elements
    /// - All elements the range `start..(start+len)` are valid
    #[inline(always)]
    pub unsafe fn slice_mut(&mut self, start: usize, len: usize) -> &mut [T] {
        unsafe {
            // SAFETY: Similar to safety notes for `slice`, but we have a mutable reference
            // which is also guaranteed to be valid for writes.
            // Ref: https://doc.rust-lang.org/stable/src/core/mem/maybe_uninit.rs.html#994
            slice::from_raw_parts_mut(self.0.as_ptr().add(start) as *mut T, len)
        }
    }

    /// Drops the element at `index` and replaces it with the last element in `0..len`.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the following:
    /// - This pointer has allocated at least `len` elements
    /// - All elements the range `0..len` are valid
    /// - `len > 0`
    /// - `index < len`
    #[inline(always)]
    pub unsafe fn swap_remove(&mut self, index: usize, len: usize) -> T {
        unsafe {
            debug_assert!(len > 0);
            debug_assert!(index < len);

            // SAFETY: The caller is guaranteeing that the element at index, and
            // the element at len - 1 are both valid. With this guarantee we can
            // safely take the element at index. We then perform a direct pointer
            // copy (we can't assume nonoverlapping here!) from the last element
            // to the one at index. This moves the data, making the data at index
            // valid to the data at last, and the data at last invalid (even if
            // it is still bitwise identical to the data at index).
            let last = len - 1;
            let array_ptr = self.0.as_ptr();
            let result = ptr::read(array_ptr.add(index)).assume_init();
            ptr::copy(array_ptr.add(last), array_ptr.add(index), 1);
            *array_ptr.add(last) = MaybeUninit::uninit(); // Hint for Miri
            result
        }
    }

    /// Drops all elements in the range `0..len`.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the following:
    /// - All elements in the range `0..len` are valid
    /// - `len <= N`
    #[inline(always)]
    pub unsafe fn drop_to(&mut self, len: usize) {
        unsafe {
            for i in 0..len {
                let i_ptr = self.0.as_ptr().add(i);
                // SAFETY: The caller guarantees this element is valid.
                ptr::drop_in_place(i_ptr as *mut T);
                ptr::write(i_ptr, MaybeUninit::uninit()); // Hint for Miri
            }
        };
    }
}

#[inline(always)]
fn new_layout<T>(capacity: usize) -> Layout {
    let layout = Layout::array::<T>(capacity).unwrap();
    assert!(layout.size() <= isize::MAX as usize, "allocation too large");
    layout
}

#[inline(always)]
fn resolve_ptr<T>(ptr: *mut u8, layout: Layout) -> NonNull<T> {
    match NonNull::new(ptr as *mut T) {
        Some(p) => p,
        None => alloc::handle_alloc_error(layout),
    }
}
