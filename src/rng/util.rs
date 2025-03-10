pub(crate) fn u64_to_u32_high(num: u64) -> u32 {
    (num >> 32) as u32
}

pub(crate) fn u64_to_u32_low(num: u64) -> u32 {
    num as u32
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
