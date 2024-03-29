use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6];
    let r = &mut vec[..];
    let (a, b) = split_at_mut(r, 3);

    println!("A {:?}  B {:?}", a, b);
}
