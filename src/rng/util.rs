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
