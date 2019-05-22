// A small example which is primarily used to help benchmark walrus right now.

fn main() {
    env_logger::init();
    let a = std::env::args().nth(1).unwrap();
    let mut m = walrus::Module::from_file(&a).unwrap();
    let wasm = m.emit_wasm();
    if let Some(destination) = std::env::args().nth(2) {
        std::fs::write(destination, wasm).unwrap();
    }
}
