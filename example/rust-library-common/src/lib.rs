#[repr(C)]
pub struct Object {
    pub value: i64,
}

#[unsafe(no_mangle)]
pub extern "C" fn create_object(value: i64) -> *mut Object {
    Box::into_raw(Box::new(Object { value }))
}
