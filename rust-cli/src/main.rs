const USAGE_STR: &str = "wasm-share-memory <input.wasm> <output.wasm> <offset> <heap_base>";

fn main() {
    let mut args = std::env::args();

    let input_file = args.nth(1).expect(USAGE_STR);
    let output_file = args.next().expect(USAGE_STR);
    let offset = args
        .next()
        .expect(USAGE_STR)
        .parse::<i32>()
        .expect("offset must be a 32-bit integer");

    let hb = args
        .next()
        .expect(USAGE_STR)
        .parse::<i32>()
        .expect("heap_base must be a 32-bit integer");

    let wasm = std::fs::read(input_file).unwrap();

    let heap_base = wasm_share_memory::get_heap_base(&wasm);
    println!("Heap Base: {heap_base}");

    let patched = wasm_share_memory::relocate(&wasm, offset, hb);

    std::fs::write(output_file, patched).expect("Failed to write wasm file.")
}
