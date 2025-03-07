pub(crate) fn u64_to_u32_high(num: u64) -> u32 {
    u64_to_u32(num, true)
}

pub(crate) fn u64_to_u32_low(num: u64) -> u32 {
    u64_to_u32(num, false)
}

fn u64_to_u32(num: u64, from_high: bool) -> u32 {
    let bytes = num.to_ne_bytes();
    let (mut high, mut low) = bytes.split_at(4);

    if cfg!(target_endian = "little") {
        std::mem::swap(&mut high, &mut low);
    }
    if from_high {
        u32::from_ne_bytes(high.try_into().unwrap())
    } else {
        u32::from_ne_bytes(low.try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_high_to_low_works() {
        let num = 0x2b47fed88766bb05_u64;
        let h = u64_to_u32_high(num);
        let l = u64_to_u32_low(num);
        assert_eq!(h, 0x2b47fed8);
        assert_eq!(l, 0x8766bb05);
    }
}
