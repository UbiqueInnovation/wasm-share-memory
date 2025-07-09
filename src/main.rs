const USAGE_STR: &str = "wasm-multi-loader <input.wasm> <output.wasm> <offset>";

fn main() {
    let mut args = std::env::args();

    let input_file = args.nth(1).expect(USAGE_STR);
    let output_file = args.next().expect(USAGE_STR);
    let offset = args
        .next()
        .expect(USAGE_STR)
        .parse::<i32>()
        .expect("offset must be a 32-bit integer");

    let wasm = std::fs::read(input_file).unwrap();

    let patched = wasm_multi_loader::relocate(&wasm, offset);

    std::fs::write(output_file, patched).expect("Failed to write wasm file.")
}
