use core::mem::MaybeUninit;

/// Initializes an array using a closure.
/// The closure is called with the index of each element.
/// If error occurs, this function returns the error and drops incomplete array.
/// Otherwise, it returns the initialized array.
#[inline]
pub(crate) fn try_array_init<E, T, const N: usize>(
    f: impl FnMut(usize) -> Result<T, E>,
) -> Result<[T; N], E> {
    #![allow(unsafe_code)]

    let mut array = [const { MaybeUninit::uninit() }; N];
    if let Err(err) = try_slice_init(&mut array, f) {
        return Err(err);
    }

    // # SAFETY:
    // * All elements are initialized.
    // * `MaybeUninit<T>` and T are guaranteed to have the same layout
    // * `MaybeUninit` does not drop, so there are no double-frees
    //
    // I'd prefer MaybeUninit::array_assume_init, but it is not stable yet.
    Ok(unsafe { core::mem::transmute_copy::<_, [T; N]>(&array) })
}

/// Initializes an array using a closure.
/// The closure is called with the index of each element.
/// If error occurs, this function returns the error and drops incomplete array.
/// Otherwise, it returns the initialized array.
#[inline]
pub(crate) fn try_slice_init<E, T>(
    slice: &mut [MaybeUninit<T>],
    f: impl FnMut(usize) -> Result<T, E>,
) -> Result<&mut [T], E> {
    #![allow(unsafe_code)]

    struct Guard<'a, T> {
        slice: &'a mut [MaybeUninit<T>],
        initialized: usize,
    }

    impl<'a, T> Guard<'a, T> {
        fn is_full(&self) -> bool {
            self.initialized == self.slice.len()
        }

        unsafe fn push_unchecked(&mut self, value: T) {
            self.slice.get_unchecked_mut(self.initialized).write(value);
            self.initialized += 1;
        }
    }

    impl<'a, T> Drop for Guard<'a, T> {
        fn drop(&mut self) {
            // # SAFETY: The slice is not fully initialized, but we may drop initialized elements.
            unsafe {
                core::ptr::drop_in_place(core::ptr::slice_from_raw_parts_mut(
                    self.slice.as_mut_ptr(),
                    self.initialized,
                ))
            };
        }
    }

    let mut guard = Guard {
        slice,
        initialized: 0,
    };

    let mut f = f;

    while !guard.is_full() {
        let index = guard.initialized;
        match f(index) {
            Ok(value) => unsafe {
                guard.push_unchecked(value);
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    // Do not drop the guard, as it will drop the initialized elements.
    core::mem::forget(guard);

    // # SAFETY: All elements are initialized.
    Ok(unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) })
}
