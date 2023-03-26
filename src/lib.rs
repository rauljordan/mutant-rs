#[link(name = "zig", kind = "static")]
extern "C" {
    fn mutate_ffi(buf: *mut u8, len: usize, num_mutations: usize);
}

/// Mutator creates an instance of a byte vector mutator that
/// can perform different strategies to modify its input into random bytes
/// using Zig for lightning fast speeds and minimal memory allocations.
pub struct Mutator {
    input: Vec<u8>,
    num_iters: usize,
    seed: u64,
}

impl Default for Mutator {
    fn default() -> Self {
        Self {
            input: Vec::new(),
            num_iters: 100,
            seed: 2394923094234,
        }
    }
}

impl Mutator {
    pub fn new() -> Self {
        Mutator::default()
    }
    /// The input byte vector to the mutator.
    pub fn input(mut self, input: Vec<u8>) -> Mutator {
        self.input = input;
        self
    }
    /// The seed used for randomizing the mutator's bytes.
    pub fn seed(mut self, seed: u64) -> Mutator {
        self.seed = seed;
        self
    }
    /// The number of mutations performed on the byte vector.
    pub fn num_iters(mut self, n: usize) -> Mutator {
        self.num_iters = n;
        self
    }
    /// Mutate the input buffer.
    pub fn mutate(&mut self) {
        let len = self.input.len();
        let ptr = self.input.as_mut_ptr();

        unsafe {
            mutate_ffi(ptr, len, self.num_iters);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let buf = vec![0; 6];
        let mut mutator = Mutator::new().input(buf).num_iters(1_000_000);
        let start = std::time::Instant::now();
        mutator.mutate();
        println!("took={:?} for Zig to respond via FFI", start.elapsed());
    }
}
