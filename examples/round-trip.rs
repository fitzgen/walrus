//! A small example which is primarily used to help benchmark walrus right now.

use std::process;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error!");
        for c in e.iter_chain() {
            eprintln!("{}", c);
        }
        process::exit(1);
    }
}

fn try_main() -> Result<(), failure::Error> {
    env_logger::init();

    rayon::ThreadPoolBuilder::new()
        .num_threads(48)
        .spawn_handler(|thread| {
            std::thread::spawn(|| {
                coz::thread_init();
                thread.run()
            });
            Ok(())
        })
        .build_global()
        .unwrap();

    let a = std::env::args().nth(1).ok_or_else(|| {
        failure::format_err!("must provide the input wasm file as the first argument")
    })?;
    let n = std::env::args().nth(2).unwrap_or("100".into());
    let n = n.parse::<usize>()?;

    let buf = std::fs::read(a)?;

    for _ in 0..n {
        coz::begin!("round-trip");

        // coz::begin!("parse");
        let mut m = walrus::Module::from_buffer(&buf)?;
        // coz::end!("parse");

        // coz::begin!("gc");
        walrus::passes::gc::run(&mut m);
        // coz::end!("gc");

        // coz::begin!("emit");
        let wasm = m.emit_wasm();
        // coz::end!("emit");

        coz::end!("round-trip");

        if let Some(destination) = std::env::args().nth(2) {
            std::fs::write(destination, wasm)?;
        }
    }

    Ok(())
}
