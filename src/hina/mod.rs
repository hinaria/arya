//! excerpts from `crate hina`.



use {
    std,
};



// convert a singular `T` into a single element slice of `T`.
crate fn as_slice<T>(item: &T) -> &[T] {
    unsafe {
        std::slice::from_raw_parts(item, 1)
    }
}
