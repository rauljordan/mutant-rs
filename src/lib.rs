#[link(name = "zig", kind = "static")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

pub fn do_add(a: i32, b: i32) -> i32 {
    println!("Hi");
    unsafe { add(a, b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = unsafe { add(2, 2) };
        assert_eq!(result, 4);
    }
}
