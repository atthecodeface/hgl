//fi as_u8s
pub unsafe fn as_u8s<T: Sized>(obj: &T) -> &[u8] {
    std::slice::from_raw_parts((obj as *const T).cast::<u8>(), size_of::<T>())
}

//fi as_u8s_mut
pub unsafe fn as_u8s_mut<T: Sized>(obj: &mut T) -> &mut [u8] {
    std::slice::from_raw_parts_mut((obj as *mut T).cast::<u8>(), size_of::<T>())
}
