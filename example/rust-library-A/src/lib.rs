use common::Object;

#[unsafe(no_mangle)]
pub extern "C" fn get_value(object: &Object) -> i64 {
    object.value
}
