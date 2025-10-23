// Re-export the luby_transform module
pub mod luby_transform;

// Re-export PRNG and related functions for backward compatibility
pub use luby_transform::prng::PRNG;
pub use luby_transform::prng::{gen_tau, gen_rho, gen_mu, gen_rsd_cdf, DEFAULT_C, DEFAULT_DELTA};
pub use luby_transform::ltencoder::LtEncoder;
pub use luby_transform::ltdecoder::LtDecoder;
