#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut u8, capacity: usize) {
    let _ = unsafe { Vec::from_raw_parts(ptr, 0, capacity) };
}
