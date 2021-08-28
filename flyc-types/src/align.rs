/// Rounds `x` to the nearest multiple of `mul`.
pub fn round(x: usize, mul: usize) -> usize {
    ((x + mul - 1) / mul) * mul
}
