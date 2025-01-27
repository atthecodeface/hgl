//fp as_u8s
/// # Safety
///
/// This function reinterprets obj as a slice of u8; it is free to
/// interpret that data in a manner that breaks the validity of T, so
/// it should be used with caution
///
/// It is designed to be used to access a bit-copyable data value
pub unsafe fn as_u8s<T: Sized + Copy>(obj: &T) -> &[u8] {
    std::slice::from_raw_parts((obj as *const T).cast::<u8>(), size_of::<T>())
}

//fp as_u8s_mut
/// # Safety
///
/// This function reinterprets obj as a mutable slice of u8; a user is free to
/// change that data as much as they like. It must be used with extreme caution
///
/// It is designed to provide mutable access a bit-copyable data value
pub unsafe fn as_u8s_mut<T: Sized + Copy>(obj: &mut T) -> &mut [u8] {
    std::slice::from_raw_parts_mut((obj as *mut T).cast::<u8>(), size_of::<T>())
}
