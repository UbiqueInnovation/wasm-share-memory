use std::collections::HashMap;

use walrus::{ConstExpr, DataKind, ExportItem, GlobalKind, Module, ir::Value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn relocate(module: &[u8], offset: i32) -> Vec<u8> {
    let mut module = Module::from_buffer(module).expect("Failed to parse WASM module.");

    let globals = module
        .globals
        .iter()
        .filter_map(|it| it.name.as_ref().map(|name| (name.clone(), it.id())))
        .collect::<HashMap<_, _>>();

    // __memory_base
    if let Some(id) = globals.get("GOT.data.internal.__memory_base") {
        module.globals.get_mut(id.clone()).kind =
            GlobalKind::Local(ConstExpr::Value(Value::I32(offset)));
    } else {
        println!("WASM did not contain a __memory_base");
    }

    // __stack_pointer
    if let Some(id) = globals.get("__stack_pointer") {
        if let GlobalKind::Local(ConstExpr::Value(Value::I32(value))) =
            &mut module.globals.get_mut(id.clone()).kind
        {
            *value += offset;
        } else {
            println!("__stack_pointer was not an i32 value");
        }
    } else {
        println!("WASM did not contain a __stack_pointer");
    }

    let ids = module.data.iter().map(|it| it.id()).collect::<Vec<_>>();

    for id in ids {
        let data = module.data.get_mut(id);

        let DataKind::Active {
            memory: _,
            offset: pos,
        } = &mut data.kind
        else {
            continue;
        };

        let ConstExpr::Value(Value::I32(value)) = pos else {
            continue;
        };

        *value += offset;
    }

    module.emit_wasm()
}

#[wasm_bindgen]
pub fn get_heap_base(module: &[u8]) -> i32 {
    let mut module = Module::from_buffer(module).expect("Failed to parse WASM module.");

    let exports = module
        .exports
        .iter()
        .map(|it| (it.name.clone(), it.id()))
        .collect::<HashMap<_, _>>();

    let id = exports
        .get("__heap_base")
        .expect("WASM did not countain a __heap_base");

    let ExportItem::Global(gid) = module.exports.get_mut(id.clone()).item else {
        panic!("__heap_base global id not found")
    };

    let GlobalKind::Local(ConstExpr::Value(Value::I32(heap_base))) = module.globals.get(gid).kind
    else {
        panic!("__heap_base global is not a 32-bit integer")
    };

    heap_base
}
