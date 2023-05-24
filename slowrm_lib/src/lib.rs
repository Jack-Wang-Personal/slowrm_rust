// mod size;

#[derive(Debug)]
#[non_exhaustive] // Intentionally not exhaustive to allow extension of the config
pub struct SlowRm {
    /// Rate of removal (in bytes per second)
    pub rate: usize,

    /// Number of chunk removal per second
    /// 
    /// The field is used to determine the size of the individual chunk removal.
    /// Having an high value allows for a more smooth and consistent removal
    /// but it might add some more load on the drives.
    pub chunk_removal_per_second: usize,
}

impl SlowRm {
    /// Determine how many bytes should be removed in a single round
    fn max_size_per_chunk_removal(&self) -> usize {
        if self.rate < self.chunk_removal_per_second {
            self.rate
        } else {
            dbg![dbg![self.rate] / dbg![self.chunk_removal_per_second]]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SlowRm;

    #[test]
    fn max_size_per_chunk_removal() {
        assert_eq!(10, SlowRm {rate: 10, chunk_removal_per_second: 1000}.max_size_per_chunk_removal());

        assert_eq!(128 * 1024 * 1024 * 1024/1000, SlowRm {rate: 128 * 1024 * 1024 * 1024, chunk_removal_per_second: 1000}.max_size_per_chunk_removal());
    }
}