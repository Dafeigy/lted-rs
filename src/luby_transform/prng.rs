use std::collections::HashSet;

// Default parameters
pub const DEFAULT_C: f64 = 0.1;
pub const DEFAULT_DELTA: f64 = 0.5;

// Parameters for Pseudorandom Number Generator
const PRNG_A: i64 = 16807;
const PRNG_M: i64 = (1 << 31) - 1;
const PRNG_MAX_RAND: i64 = PRNG_M - 1;

/// Generates the Robust part of the RSD (tau)
pub fn gen_tau(s: f64, k: usize, delta: f64) -> Vec<f64> {
    // Ensure pivot is at least 2 to avoid overflow and negative values
    let pivot = std::cmp::max(2, std::cmp::min((k as f64 / s).floor() as usize, k - 1));
    let mut tau = Vec::with_capacity(k);
    
    // First part: 1 to pivot-1
    for d in 1..pivot {
        tau.push(s / (k as f64) * (1.0 / d as f64));
    }
    
    // Pivot point - make sure we don't exceed k elements
    if tau.len() < k {
        tau.push(s / (k as f64) * (s / delta).ln());
    }
    
    // Remaining zeros - ensure we don't create a negative size
    let remaining = k.saturating_sub(tau.len());
    if remaining > 0 {
        tau.extend(vec![0.0; remaining]);
    }
    
    tau
}

/// Generates the Ideal Soliton Distribution (rho)
pub fn gen_rho(k: usize) -> Vec<f64> {
    let mut rho = Vec::with_capacity(k);
    
    // First element is 1/K
    rho.push(1.0 / k as f64);
    
    // For d from 2 to K
    for d in 2..=k {
        rho.push(1.0 / (d as f64 * (d - 1) as f64));
    }
    
    rho
}

/// Generates the Robust Soliton Distribution (mu)
pub fn gen_mu(k: usize, delta: f64, c: f64) -> Vec<f64> {
    let s = c * ((k as f64 / delta).ln()) * (k as f64).sqrt();
    let tau = gen_tau(s, k, delta);
    let rho = gen_rho(k);
    
    // Calculate normalizer
    let normalizer = rho.iter().sum::<f64>() + tau.iter().sum::<f64>();
    
    // Combine and normalize
    rho.iter()
        .zip(tau.iter())
        .map(|(&r, &t)| (r + t) / normalizer)
        .collect()
}

/// Generates the CDF of the RSD for sampling
pub fn gen_rsd_cdf(k: usize, delta: f64, c: f64) -> Vec<f64> {
    let mu = gen_mu(k, delta, c);
    let mut cdf = Vec::with_capacity(k);
    let mut sum = 0.0;
    
    for &prob in &mu {
        sum += prob;
        cdf.push(sum);
    }
    
    cdf
}

/// A Pseudorandom Number Generator that yields samples
/// from the set of source blocks using the RSD degree
/// distribution.
pub struct PRNG {
    state: Option<i64>,
    k: usize,
    cdf: Vec<f64>,
}

impl PRNG {
    /// Creates a new PRNG with the given parameters
    pub fn new(k: usize, delta: f64, c: f64) -> Self {
        let cdf = gen_rsd_cdf(k, delta, c);
        Self {
            state: None,
            k,
            cdf,
        }
    }
    
    /// Creates a new PRNG with default parameters
    pub fn new_default(k: usize) -> Self {
        Self::new(k, DEFAULT_DELTA, DEFAULT_C)
    }
    
    /// Executes the next iteration of the PRNG
    /// evolution process, and returns the result
    fn get_next(&mut self) -> i64 {
        if let Some(current_state) = self.state {
            self.state = Some(PRNG_A * current_state % PRNG_M);
            self.state.unwrap()
        } else {
            panic!("PRNG state not initialized. Call set_seed first.")
        }
    }
    
    /// Samples degree given the precomputed
    /// distributions and the linear PRNG output
    fn sample_d(&mut self) -> usize {
        let p = self.get_next() as f64 / PRNG_MAX_RAND as f64;
        
        for (ix, &v) in self.cdf.iter().enumerate() {
            if v > p {
                return ix + 1; // degrees are 1-indexed
            }
        }
        
        self.cdf.len() // fallback to max degree
    }
    
    /// Reset the state of the PRNG to the
    /// given seed
    pub fn set_seed(&mut self, seed: i64) {
        self.state = Some(seed);
    }
    
    /// Returns the indices of a set of `d` source blocks
    /// sampled from indices i = 0, ..., K-1 uniformly, where
    /// `d` is sampled from the RSD described above.
    pub fn get_src_blocks(&mut self, seed: Option<i64>) -> (i64, usize, HashSet<usize>) {
        // Set seed if provided
        if let Some(s) = seed {
            self.state = Some(s);
        }
        
        // Check if state is initialized
        let blockseed = self.state.expect("PRNG state not initialized");
        
        // Sample degree
        let d = self.sample_d();
        
        // Sample d unique blocks
        let mut nums = HashSet::with_capacity(d);
        while nums.len() < d {
            let num = (self.get_next() % self.k as i64).abs() as usize;
            nums.insert(num);
        }
        
        (blockseed, d, nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prng_initialization() {
        let prng = PRNG::new(100, 0.5, 0.1);
        assert_eq!(prng.k, 100);
        assert_eq!(prng.cdf.len(), 100);
    }
    
    #[test]
    fn test_prng_with_seed() {
        let mut prng = PRNG::new(100, 0.5, 0.1);
        prng.set_seed(42);
        let (blockseed, d, blocks) = prng.get_src_blocks(None);
        assert_eq!(blockseed, 42);
        assert!(d >= 1 && d <= 100);
        assert_eq!(blocks.len(), d);
    }
    
    #[test]
    fn test_prng_with_direct_seed() {
        let mut prng = PRNG::new(100, 0.5, 0.1);
        let (blockseed, d, blocks) = prng.get_src_blocks(Some(42));
        assert_eq!(blockseed, 42);
        assert!(d >= 1 && d <= 100);
        assert_eq!(blocks.len(), d);
    }
}