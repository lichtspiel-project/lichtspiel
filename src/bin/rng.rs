use std::io::Write;

use lichtspiel::rng::RNG;
use lichtspiel::rng::{pcg::PCG32, xorshift::XorshiftStar32};

fn main() -> std::io::Result<()> {
    let alg = std::env::args().nth(1).expect("No algorithm given");
    let bx = std::env::args().nth(2);

    let mut rng = match alg.as_str() {
        "xor" => RNG::XorshiftStar(XorshiftStar32::new()),
        "pcg" => RNG::PCG32(PCG32::new()),
        _ => panic!("Do not understand algorithm"),
    };

    let bitmix = match bx {
        Some(flag) => flag == "--bitmix",
        _ => false,
    };

    if bitmix {
        loop {
            std::io::stdout().write_all(&rng.random_bitmix().to_ne_bytes())?;
        }
    } else {
        loop {
            std::io::stdout().write_all(&rng.random().to_ne_bytes())?;
        }
    }
}
