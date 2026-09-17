mod error;
mod extractor;
mod pool;
mod report;

pub use error::EntropyError;
pub use extractor::{analyze, extract};
pub use pool::EntropyPool;
pub use report::EntropyReport;

pub fn generate(input: &[u8], length: usize) -> Result<Vec<u8>, EntropyError> {
    let mut pool = EntropyPool::from_system()?;
    pool.mix(input)?;
    pool.bytes(length)
}

pub fn generate_deterministic(
    seed: &[u8],
    input: &[u8],
    length: usize,
) -> Result<Vec<u8>, EntropyError> {
    let mut pool = EntropyPool::from_seed(seed);
    pool.mix(input)?;
    pool.bytes(length)
}
