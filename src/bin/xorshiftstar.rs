use std::io::Write;

use lichtspiel::rng::xorshift::XorshiftStar32;

fn main() -> std::io::Result<()> {
    let mut rng = XorshiftStar32::new();

    loop {
        std::io::stdout().write_all(&rng.random().to_ne_bytes())?;
    }
}
