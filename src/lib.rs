#[link(name = "zig", kind = "static")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
    fn mutate_ffi(buf: *mut u8, len: usize);
}

pub fn do_add(a: i32, b: i32) -> i32 {
    println!("Hi");
    unsafe { add(a, b) }
}

pub fn do_mutate(buf: &mut [u8]) {
    let len = buf.len();
    let ptr = buf.as_mut_ptr();

    unsafe {
        mutate_ffi(ptr, len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut buff = vec![0; 6];
        do_mutate(&mut buff);
        println!("{:?}", std::str::from_utf8(&buff));
    }
}
