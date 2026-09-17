#[derive(Debug, Clone, PartialEq)]
pub struct EntropyReport {
    pub input_bytes: usize,
    pub unbiased_bits: usize,
    pub ones_ratio: f64,
}
