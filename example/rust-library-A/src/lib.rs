use common::Object;

#[unsafe(no_mangle)]
pub extern "C" fn get_value(object: &Object) -> i64 {
    object.value
}

#[unsafe(no_mangle)]
pub extern "C" fn create_dummy() -> *mut Object {
    Box::into_raw(Box::new(Object { value: 0 }))
}
