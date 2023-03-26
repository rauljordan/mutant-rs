#[link(name = "zig", kind = "static")]
extern "C" {
    fn mutate_ffi(buf: *mut u8, len: usize, num_mutations: usize);
}

pub fn mutate(buf: &mut [u8], num_mutations: usize) {
    let len = buf.len();
    let ptr = buf.as_mut_ptr();

    unsafe {
        mutate_ffi(ptr, len, num_mutations);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut buf = vec![0; 6];
        let start = std::time::Instant::now();
        mutate(&mut buf, 1_000_000);
        println!("{:?}", start.elapsed());
    }
}
