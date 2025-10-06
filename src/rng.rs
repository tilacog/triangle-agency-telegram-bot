use chrono::Utc;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use sha2::Digest;

/// RNG picked for the triangle agency bot.
pub type TriangleAgencyRng = ChaCha20Rng;

/// Creates an RNG from giving seed feed input.
///
/// The input of arbitrary is used to produce a seed of the correct length.
/// Additionally, timestamp data is appended to the seed feed input.
pub fn create_rng(seed_feed: impl Into<Vec<u8>>) -> TriangleAgencyRng {
    let mut seed_feed = seed_feed.into();
    let now = Utc::now();
    let nanos = now.timestamp_subsec_nanos();
    let seconds = now.timestamp();
    seed_feed.extend_from_slice(&nanos.to_le_bytes());
    seed_feed.extend_from_slice(&seconds.to_le_bytes());
    let mut seed_hasher = sha2::Sha256::new();
    seed_hasher.update(seed_feed);
    let seed = seed_hasher.finalize().into();
    TriangleAgencyRng::from_seed(seed)
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::rng::create_rng;

    #[test]
    fn rng_entropy_preserved_by_changing_input_byte() {
        let mut rng = create_rng([1_u8]);
        let byte0 = rng.random::<u8>();
        let mut rng = create_rng([0_u8]);
        let byte1 = rng.random::<u8>();
        assert_ne!(byte0, byte1);
    }

    #[test]
    fn rng_entropy_preserved_by_changing_input_length() {
        let mut rng = create_rng([1_u8]);
        let byte0 = rng.random::<u8>();
        let mut rng = create_rng([1_u8, 2_u8]);
        let byte1 = rng.random::<u8>();
        assert_ne!(byte0, byte1);
    }
}
