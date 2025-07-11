use walrus::{
    ConstExpr, FunctionKind, GlobalKind, Module,
    ir::{Instr, Value},
};
use wasm_bindgen::prelude::*;

mod symbols {
    pub const HEAP_BASE: &str = "__heap_base";
    pub const STACK_POINTER: &str = "__stack_pointer";

    pub const MEMORY_BASE: &str = "GOT.data.internal.__memory_base";

    pub const WASM_INIT_MEMORY: &str = "__wasm_init_memory";
}

mod utils {
    use std::collections::HashMap;

    use walrus::{
        ConstExpr, DataKind, ExportId, ExportItem, GlobalId, GlobalKind, Module, ir::Value,
    };

    pub fn get_named_globals_map(module: &Module) -> HashMap<String, GlobalId> {
        module
            .globals
            .iter()
            .filter_map(|it| it.name.as_ref().map(|name| (name.clone(), it.id())))
            .collect()
    }

    pub fn get_exports_map(module: &Module) -> HashMap<String, ExportId> {
        module
            .exports
            .iter()
            .map(|it| (it.name.clone(), it.id()))
            .collect()
    }

    pub fn get_global(
        module: &Module,
        globals: &HashMap<String, GlobalId>,
        symbol: &str,
    ) -> Option<i32> {
        let id = globals.get(symbol)?;

        match module.globals.get(id.clone()).kind {
            GlobalKind::Local(ConstExpr::Value(Value::I32(value))) => Some(value),
            _ => None,
        }
    }

    pub fn get_global_mut<'a>(
        module: &'a mut Module,
        globals: &HashMap<String, GlobalId>,
        symbol: &str,
    ) -> Option<&'a mut i32> {
        let id = globals.get(symbol)?;

        if let GlobalKind::Local(ConstExpr::Value(Value::I32(value))) =
            &mut module.globals.get_mut(id.clone()).kind
        {
            return Some(value);
        }

        None
    }

    pub fn get_global_from_exports(
        module: &Module,
        exports: &HashMap<String, ExportId>,
        symbol: &str,
    ) -> Option<i32> {
        let id = exports.get(symbol)?;

        let ExportItem::Global(gid) = module.exports.get(id.clone()).item else {
            return None;
        };

        match module.globals.get(gid).kind {
            GlobalKind::Local(ConstExpr::Value(Value::I32(heap_base))) => Some(heap_base),
            _ => None,
        }
    }

    pub fn get_global_from_exports_mut<'a>(
        module: &'a mut Module,
        exports: &HashMap<String, ExportId>,
        symbol: &str,
    ) -> Option<&'a mut i32> {
        let id = exports.get(symbol)?;

        let ExportItem::Global(gid) = module.exports.get(id.clone()).item else {
            return None;
        };

        match &mut module.globals.get_mut(gid).kind {
            GlobalKind::Local(ConstExpr::Value(Value::I32(heap_base))) => Some(heap_base),
            _ => None,
        }
    }

    pub fn get_segment_position_mut(
        module: &mut Module,
        segment_id: walrus::DataId,
    ) -> Option<&mut i32> {
        let data = module.data.get_mut(segment_id);

        let DataKind::Active { offset, .. } = &mut data.kind else {
            return None;
        };

        match offset {
            ConstExpr::Value(Value::I32(value)) => Some(value),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub fn relocate(module: &[u8], offset: i32, new_heap_base: i32) -> Vec<u8> {
    let mut module = Module::from_buffer(module).expect("Failed to parse WASM module.");

    let globals = utils::get_named_globals_map(&module);
    let exports = utils::get_exports_map(&module);

    let heap_base = utils::get_global_from_exports(&module, &exports, symbols::HEAP_BASE)
        .expect("Failed to get heap base");

    let stack_pointer = utils::get_global(&module, &globals, symbols::STACK_POINTER)
        .expect("Failed to get stack pointer");

    let memory_base = utils::get_global(&module, &globals, symbols::MEMORY_BASE)
        .expect("Failed to get memory base");

    // Patch __wasm_init_memory function
    {
        let id = module
            .funcs
            .by_name(symbols::WASM_INIT_MEMORY)
            .expect("Failed to find __wasm_init_memory function");

        let func = module.funcs.get_mut(id);

        let FunctionKind::Local(func) = &mut func.kind else {
            panic!("Expected __wasm_init_memory to be a local function");
        };

        let mut body = func.builder_mut().func_body();

        let instrs = body.instrs_mut();

        assert_eq!(
            instrs.len(),
            4,
            "Expected __wasm_init_memory to have 4 instructions"
        );

        let (first, _) = instrs.first_mut().unwrap();

        let Instr::Const(instr) = first else {
            panic!("Expected first instruction of __wasm_init_memory to be a Const");
        };

        let Value::I32(value) = &mut instr.value else {
            panic!("Expected first instruction of __wasm_init_memory to be an i32 Const");
        };

        assert!(
            stack_pointer <= *value && *value < heap_base,
            "Expected first instruction of __wasm_init_memory to be within stack and heap bounds"
        );

        *value += offset;
    }

    // Patch all globals between __stack_pointer and __heap_base
    for id in module.globals.iter().map(|it| it.id()).collect::<Vec<_>>() {
        let GlobalKind::Local(ConstExpr::Value(Value::I32(address))) =
            &mut module.globals.get_mut(id).kind
        else {
            // addresses are always i32, safe to skip other types
            continue;
        };

        // stack <= address <= heap      (metadata is stored here)
        if stack_pointer <= *address && *address <= heap_base {
            *address += offset;
        }
    }

    // Patch all segments
    {
        let segments = module.data.iter().map(|it| it.id()).collect::<Vec<_>>();

        for id in segments {
            let position = utils::get_segment_position_mut(&mut module, id)
                .expect("Failed to get segment position");

            *position += offset;
        }
    }

    // Patch __memory_base, __stack_pointer, __heap_base
    {
        let memory_base_mut = utils::get_global_mut(&mut module, &globals, symbols::MEMORY_BASE)
            .expect("Failed to get memory base");
        *memory_base_mut = memory_base + offset;

        let stack_pointer_mut =
            utils::get_global_mut(&mut module, &globals, symbols::STACK_POINTER)
                .expect("Failed to get stack pointer");
        *stack_pointer_mut = stack_pointer + offset;

        let heap_base =
            utils::get_global_from_exports_mut(&mut module, &exports, symbols::HEAP_BASE)
                .expect("Failed to get heap base");
        *heap_base = new_heap_base;
    }

    module.emit_wasm()
}

#[wasm_bindgen]
pub fn get_heap_base(module: &[u8]) -> i32 {
    let module = Module::from_buffer(module).expect("Failed to parse WASM module.");

    let exports = utils::get_exports_map(&module);

    utils::get_global_from_exports(&module, &exports, symbols::HEAP_BASE)
        .expect("Failed to get heap base")
}
