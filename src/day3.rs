use super::*;

static DIGIT_LUT: [u8x16; 1 << 7] =
    unsafe { std::mem::transmute(*include_bytes!("day3-digit.bin")) };
static SEP_LUT: [u8x16; 1 << 8] = unsafe { std::mem::transmute(*include_bytes!("day3-sep.bin")) };

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn count(s: &str) -> u64 {
    let mut ptr = s.as_bytes().as_ptr();
    let end = ptr.add(s.len());
    let m: u8x32 = Simd::splat('m' as u8);
    let u: u8x32 = Simd::splat('u' as u8);
    let l: u8x32 = Simd::splat('l' as u8);
    let p: u8x32 = Simd::splat('(' as u8);
    let ten: u8x16 = Simd::splat(10);
    let ascii_zero: u8x16 = Simd::splat('0' as u8);
    const CONSTANT: u64 = ',' as u64 | ((')' as u64) << 32);
    let seps: u64x2 = Simd::from_array([CONSTANT, 0]);
    let mul2: u8x16 =
        Simd::from_array([1, 10, 100, 0, 1, 10, 100, 0, 1, 10, 100, 0, 1, 10, 100, 0]);
    let mut sum: u64x2 = Simd::splat(0);

    loop {
        let a = (ptr.add(0) as *const u8x32).read_unaligned();
        let b = (ptr.add(1) as *const u8x32).read_unaligned();
        let c = (ptr.add(2) as *const u8x32).read_unaligned();
        let d = (ptr.add(3) as *const u8x32).read_unaligned();

        let mask = a.simd_eq(m).to_bitmask()
            & b.simd_eq(u).to_bitmask()
            & c.simd_eq(l).to_bitmask()
            & d.simd_eq(p).to_bitmask();
        let idx = (mask as u32).trailing_zeros();

        let offset = idx as usize % 32 + 4;
        let part = (ptr.add(offset) as *const u8x16).read_unaligned();
        let digits = part - ascii_zero;

        let notfound = (idx == 32) as usize;
        let found = (idx != 32) as usize;
        let next = 32 * notfound + offset * found as usize;

        // n n n , n n n )
        // 1 2 3 4 5 6 7 8
        let positions = digits.simd_lt(ten).to_bitmask() as usize & 0b01111111;
        let separators = !positions & 0xff;
        let shuffled: u8x16 = _mm_shuffle_epi8(digits.into(), DIGIT_LUT[positions].into()).into();
        let test: u32x4 = _mm_shuffle_epi8(part.into(), SEP_LUT[separators].into()).into();
        let valid: u64x2 = _mm_cmpeq_epi64(test.into(), seps.into()).into();

        let digit2: u16x8 = _mm_maddubs_epi16(shuffled.into(), mul2.into()).into();
        let digit3: u16x8 = _mm_hadd_epi16(digit2.into(), digit2.into()).into();
        let nums: u32x4 = _mm_cvtepi16_epi32(digit3.into()).into();
        let other: u16x8 = _mm_srli_epi64::<32>(digit3.into()).into();
        let finish: u64x2 = _mm_mul_epi32(nums.into(), other.into()).into();

        sum += finish & valid;

        ptr = ptr.add(next);
        if ptr >= end {
            return sum[0] as u64;
        }
    }
}

pub fn part1(s: &str) -> impl Display {
    unsafe { count(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn switch(s: &str) -> u64 {
    loop {}
}

pub fn part2(s: &str) -> impl Display {
    unsafe { switch(s) }
}
