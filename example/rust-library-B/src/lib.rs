use common::Object;

#[unsafe(no_mangle)]
pub extern "C" fn double(object: &mut Object) {
    object.value *= 2;
}
