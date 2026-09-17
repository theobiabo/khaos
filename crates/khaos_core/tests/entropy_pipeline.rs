use khaos_core::{analyze, extract, generate_deterministic, EntropyError};

const CHANGING_INPUT: &[u8] = &[0b0110_1001, 0b1001_0110, 0b0101_1010];

#[test]
fn rejects_empty_input() {
    assert_eq!(extract(&[]), Err(EntropyError::EmptyInput));
}

#[test]
fn rejects_input_without_changing_pairs() {
    assert_eq!(extract(&[0, 255]), Err(EntropyError::BiasedInput));
}

#[test]
fn deterministic_mode_replays_the_same_output() {
    let first = generate_deterministic(b"lesson-one", CHANGING_INPUT, 32).unwrap();
    let second = generate_deterministic(b"lesson-one", CHANGING_INPUT, 32).unwrap();
    assert_eq!(first, second);
}

#[test]
fn different_inputs_change_the_output() {
    let first = generate_deterministic(b"lesson-one", CHANGING_INPUT, 32).unwrap();
    let second = generate_deterministic(b"lesson-one", &[0b0110_0110], 32).unwrap();
    assert_ne!(first, second);
}

#[test]
fn reports_the_input_distribution() {
    let report = analyze(&[0b1111_0000]);
    assert_eq!(report.input_bytes, 1);
    assert_eq!(report.ones_ratio, 0.5);
}
