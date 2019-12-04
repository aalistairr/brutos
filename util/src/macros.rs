#[doc(hidden)]
#[macro_export]
macro_rules! offset_of {
    (@ $parent:ty, $field:tt) => {{
        // Create an instance of the container and calculate the offset to its field.
        // Here we're using an uninitialized instance of $parent. We avoid UB
        // by only using raw pointers that point to real (allocated, albeit uninitialized) memory.
        let val = core::mem::MaybeUninit::<$parent>::uninit();
        let base_ptr = val.as_ptr();
        #[allow(unused_unsafe)] // for when the macro is used in an unsafe block
        let field_ptr = unsafe { &(*base_ptr).$field as *const _ };
        (field_ptr as usize) - (base_ptr as usize)
    }};
    ($parent:tt, $field:tt) => {{
        // Make sure the field actually exists. This line ensures that a
        // compile-time error is generated if $field is accessed through a
        // Deref impl.
        #[allow(clippy::unneeded_field_pattern)]
        let $parent { $field: _, .. };
        $crate::offset_of!(@ $parent, $field)
    }};
}
