use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;

use super::{analyze, extract, EntropyError, EntropyReport};

const MAX_OUTPUT_LENGTH: usize = 1_048_576;

pub struct EntropyPool {
    key: [u8; 32],
    generation: u64,
}

impl EntropyPool {
    pub fn from_system() -> Result<Self, EntropyError> {
        let mut seed = [0_u8; 32];
        getrandom::fill(&mut seed).map_err(|_| EntropyError::SystemRandomnessUnavailable)?;
        Ok(Self::from_seed(&seed))
    }

    pub fn from_seed(seed: &[u8]) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"khaos pool seed v1");
        hasher.update(seed);

        Self {
            key: *hasher.finalize().as_bytes(),
            generation: 0,
        }
    }

    pub fn mix(&mut self, input: &[u8]) -> Result<EntropyReport, EntropyError> {
        let extracted = extract(input)?;
        let mut hasher = blake3::Hasher::new_keyed(&self.key);
        hasher.update(b"khaos pool mix v1");
        hasher.update(&self.generation.to_le_bytes());
        hasher.update(&extracted);
        self.key = *hasher.finalize().as_bytes();
        self.generation = self.generation.wrapping_add(1);
        Ok(analyze(input))
    }

    pub fn bytes(&mut self, length: usize) -> Result<Vec<u8>, EntropyError> {
        if length > MAX_OUTPUT_LENGTH {
            return Err(EntropyError::OutputTooLarge);
        }

        let mut nonce_hasher = blake3::Hasher::new_keyed(&self.key);
        nonce_hasher.update(b"khaos chacha20 nonce v1");
        nonce_hasher.update(&self.generation.to_le_bytes());
        let nonce_hash = nonce_hasher.finalize();
        let mut nonce = [0_u8; 12];
        nonce.copy_from_slice(&nonce_hash.as_bytes()[..12]);

        let mut output = vec![0_u8; length];
        let mut cipher = ChaCha20::new((&self.key).into(), (&nonce).into());
        cipher.apply_keystream(&mut output);

        let mut rekey = blake3::Hasher::new_keyed(&self.key);
        rekey.update(b"khaos pool rekey v1");
        rekey.update(&self.generation.to_le_bytes());
        rekey.update(&output);
        self.key = *rekey.finalize().as_bytes();
        self.generation = self.generation.wrapping_add(1);

        Ok(output)
    }
}
