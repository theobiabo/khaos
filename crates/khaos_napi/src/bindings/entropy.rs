use napi::bindgen_prelude::{Buffer, Error, Result, Status};
use napi_derive::napi;

#[napi(object)]
pub struct EntropyReport {
    pub input_bytes: u32,
    pub unbiased_bits: u32,
    pub ones_ratio: f64,
}

#[napi]
pub fn analyze_entropy(input: Buffer) -> EntropyReport {
    let report = khaos_core::analyze(&input);

    EntropyReport {
        input_bytes: report.input_bytes as u32,
        unbiased_bits: report.unbiased_bits as u32,
        ones_ratio: report.ones_ratio,
    }
}

#[napi]
pub fn generate_bytes(input: Buffer, length: u32) -> Result<Buffer> {
    khaos_core::generate(&input, length as usize)
        .map(Buffer::from)
        .map_err(to_napi_error)
}

#[napi]
pub fn replay_bytes(seed: Buffer, input: Buffer, length: u32) -> Result<Buffer> {
    khaos_core::generate_deterministic(&seed, &input, length as usize)
        .map(Buffer::from)
        .map_err(to_napi_error)
}

fn to_napi_error(error: khaos_core::EntropyError) -> Error {
    Error::new(Status::InvalidArg, error.to_string())
}
