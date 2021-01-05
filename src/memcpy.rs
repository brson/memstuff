#[inline]
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, num: usize) {
    memcpy_simple_loop_with_offset(dest, src, num)
}

pub unsafe fn memcpy_simple_loop_with_offset(dest: *mut u8, src: *const u8, num: usize) {
    debug_assert!(num <= isize::max_value() as usize);
    let num = num as isize;
    let mut offset = 0;
    while offset < num {
        let src = src.offset(offset);
        let dest = dest.offset(offset);
        *dest = *src;
        offset += 1;
    }
}

pub unsafe fn memcpy_simple_loop_with_increment(dest: *mut u8, src: *const u8, num: usize) {
    debug_assert!(num <= isize::max_value() as usize);
    let num = num as isize;
    let mut src = src;
    let mut dest = dest;
    let src_max = src.offset(num);
    while src < src_max {
        *dest = *src;
        src = src.offset(1);
        dest = dest.offset(1);
    }
}

pub unsafe fn memcpy_simple_loop_with_offset_unrolled_x4(dest: *mut u8, src: *const u8, num: usize) {
    panic!()
}
