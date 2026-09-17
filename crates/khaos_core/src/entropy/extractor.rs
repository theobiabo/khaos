use super::{EntropyError, EntropyReport};

pub fn analyze(input: &[u8]) -> EntropyReport {
    let ones = input
        .iter()
        .map(|byte| byte.count_ones() as usize)
        .sum::<usize>();
    let total_bits = input.len() * 8;
    let ones_ratio = if total_bits == 0 {
        0.0
    } else {
        ones as f64 / total_bits as f64
    };

    EntropyReport {
        input_bytes: input.len(),
        unbiased_bits: von_neumann_bits(input).len(),
        ones_ratio,
    }
}

pub fn extract(input: &[u8]) -> Result<[u8; 32], EntropyError> {
    if input.is_empty() {
        return Err(EntropyError::EmptyInput);
    }

    let unbiased_bits = von_neumann_bits(input);
    if unbiased_bits.is_empty() {
        return Err(EntropyError::BiasedInput);
    }

    let unbiased_bytes = pack_bits(&unbiased_bits);
    let mut extractor = blake3::Hasher::new();
    extractor.update(b"khaos entropy extractor v1");
    extractor.update(&(input.len() as u64).to_le_bytes());
    extractor.update(&(unbiased_bits.len() as u64).to_le_bytes());
    extractor.update(&unbiased_bytes);

    Ok(*extractor.finalize().as_bytes())
}

fn von_neumann_bits(input: &[u8]) -> Vec<u8> {
    let bits = input
        .iter()
        .flat_map(|byte| (0..8).rev().map(move |shift| (byte >> shift) & 1))
        .collect::<Vec<_>>();

    bits.chunks_exact(2)
        .filter_map(|pair| match pair {
            [0, 1] => Some(0),
            [1, 0] => Some(1),
            _ => None,
        })
        .collect()
}

fn pack_bits(bits: &[u8]) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| {
            chunk
                .iter()
                .enumerate()
                .fold(0_u8, |byte, (index, bit)| byte | (bit << (7 - index)))
        })
        .collect()
}
