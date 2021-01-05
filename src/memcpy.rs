#[inline]
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, num: usize) {
    memcpy_simple_loop_with_offset(dest, src, num)
}

#[inline]
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

#[inline]
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

#[inline]
pub unsafe fn memcpy_simple_loop_with_offset_unrolled_x4(dest: *mut u8, src: *const u8, num: usize) {
    debug_assert!(num <= isize::max_value() as usize);
    let loops = num / 4;
    let rem = num - (loops * 4);
    let mut src = src;
    let mut dest = dest;
    for _ in 0..loops {
        *dest.offset(0) = *src.offset(0);
        *dest.offset(1) = *src.offset(1);
        *dest.offset(2) = *src.offset(2);
        *dest.offset(3) = *src.offset(3);
        src = src.offset(4);
        dest = dest.offset(4);
    }
    let rem_src = src.offset((num - rem) as isize);
    let rem_dest = dest.offset((num - rem) as isize);
    memcpy_simple_loop_with_offset(rem_dest, rem_src, rem)
}

#[inline]
pub unsafe fn memcpy_simple_loop_with_offset_unrolled_x4_aligned32(dest: *mut u8, src: *const u8, num: usize) {
    let src_align_offset = src.align_offset(4);
    let dest_align_offset = dest.align_offset(4);
    if src_align_offset == dest_align_offset {
        memcpy_simple_loop_with_offset(dest, src, src_align_offset);
        let src = src.add(src_align_offset);
        let dest = dest.add(dest_align_offset);
        let num = num - src_align_offset;
        memcpy_simple_loop_with_offset_unrolled_x4(dest, src, num)
    } else {
        memcpy_simple_loop_with_offset_unrolled_x4(dest, src, num)
    }
}
