use std::io::Write;

use lichtspiel::rng::PCG32;

fn main() -> std::io::Result<()> {
    let mut rng = PCG32::new();

    loop {
        std::io::stdout().write_all(&rng.random().to_ne_bytes())?;
    }
}
