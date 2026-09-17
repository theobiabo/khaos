use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntropyError {
    EmptyInput,
    BiasedInput,
    OutputTooLarge,
    SystemRandomnessUnavailable,
}

impl Display for EntropyError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::EmptyInput => "the entropy input is empty",
            Self::BiasedInput => "the entropy input does not contain enough changing bit pairs",
            Self::OutputTooLarge => "a single request cannot exceed 1048576 bytes",
            Self::SystemRandomnessUnavailable => {
                "the operating system could not provide randomness"
            }
        };

        formatter.write_str(message)
    }
}

impl Error for EntropyError {}
