use super::*;

static DIGIT_LUT: [u8x16; 1 << 7] =
    unsafe { std::mem::transmute(*include_bytes!("day3-digit.bin")) };

#[export_name = "part1"]
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn count(s: &[u8]) -> u64 {
    let mut ptr = s.as_ptr();
    let end = ptr.add(s.len());
    let m: u8x32 = Simd::splat('m' as u8);
    let u: u8x32 = Simd::splat('u' as u8);
    let l: u8x32 = Simd::splat('l' as u8);
    let p: u8x32 = Simd::splat('(' as u8);
    let ten: u8x16 = Simd::splat(10);
    let ascii_zero: u8x16 = Simd::splat('0' as u8);
    // let seps: u64x2 = Simd::from_array([(0xfc << 24) | (0xf9 << 56), 0]);
    let seps: u64x2 = Simd::from_array([(124 << 24) | (121 << 56), 0]);
    let sep_mask: u8x16 = Simd::from_array([0, 0, 0, 0xff, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]);
    let mul2: u8x16 =
        Simd::from_array([1, 10, 100, 0, 1, 10, 100, 0, 1, 10, 100, 0, 1, 10, 100, 0]);
    const HASH: u8 = ((b'm' as u32 * 2) + b'u' as u32 + b'l' as u32 + b'(' as u32) as u8;
    let target: u8x32 = Simd::splat(HASH);
    let valid_mask: u64x2 = Simd::from_array([0xffffffffffffffff, 0]);
    let digit_mask: u8x16 = Simd::splat(0x7f);
    let range: u8x16 = Simd::from_array([0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let mut sum: u64x2 = Simd::splat(0);

    'solve: loop {
        let a0 = (ptr.add(0 + 32 * 0) as *const u8x32).read_unaligned();
        let b0 = (ptr.add(1 + 32 * 0) as *const u8x32).read_unaligned();
        let c0 = (ptr.add(2 + 32 * 0) as *const u8x32).read_unaligned();
        let d0 = (ptr.add(3 + 32 * 0) as *const u8x32).read_unaligned();

        let a1 = (ptr.add(0 + 32 * 1) as *const u8x32).read_unaligned();
        let b1 = (ptr.add(1 + 32 * 1) as *const u8x32).read_unaligned();
        let c1 = (ptr.add(2 + 32 * 1) as *const u8x32).read_unaligned();
        let d1 = (ptr.add(3 + 32 * 1) as *const u8x32).read_unaligned();

        // let a2 = (ptr.add(0 + 32 * 2) as *const u8x32).read_unaligned();
        // let b2 = (ptr.add(1 + 32 * 2) as *const u8x32).read_unaligned();
        // let c2 = (ptr.add(2 + 32 * 2) as *const u8x32).read_unaligned();
        // let d2 = (ptr.add(3 + 32 * 2) as *const u8x32).read_unaligned();

        // let a3 = (ptr.add(0 + 32 * 3) as *const u8x32).read_unaligned();
        // let b3 = (ptr.add(1 + 32 * 3) as *const u8x32).read_unaligned();
        // let c3 = (ptr.add(2 + 32 * 3) as *const u8x32).read_unaligned();
        // let d3 = (ptr.add(3 + 32 * 3) as *const u8x32).read_unaligned();

        // let b0 = a0.rotate_elements_left::<1>();
        // let c0 = a0.rotate_elements_left::<2>();
        // let d0 = a0.rotate_elements_left::<3>();
        // // println!("{:?} {:?} {:?}", b0, c0, d0);
        // let tmp0: u8x32 = _mm256_slli_epi64(a0.into(), 1).into();
        // let tmp1: u8x32 = _mm256_slli_epi64(a1.into(), 1).into();

        // let lo0 = tmp0 + b0;
        // let lo1 = tmp1 + b1;
        // let hi0 = c0 + d0;
        // let hi1 = c1 + d1;

        // let hash0 = lo0 + hi0;
        // let hash1 = lo1 + hi1;

        // let mask0 = hash0.simd_eq(target).to_bitmask();
        // let mask1 = hash1.simd_eq(target).to_bitmask();
        let mask0 = a0.simd_eq(m).to_bitmask()
            & b0.simd_eq(u).to_bitmask()
            & c0.simd_eq(l).to_bitmask()
            & d0.simd_eq(p).to_bitmask();
        let mask1 = a1.simd_eq(m).to_bitmask()
            & b1.simd_eq(u).to_bitmask()
            & c1.simd_eq(l).to_bitmask()
            & d1.simd_eq(p).to_bitmask();
        // let mask2 = a2.simd_eq(m).to_bitmask()
        //     & b2.simd_eq(u).to_bitmask()
        //     & c2.simd_eq(l).to_bitmask()
        //     & d2.simd_eq(p).to_bitmask();
        // let mask3 = a3.simd_eq(m).to_bitmask()
        //     & b3.simd_eq(u).to_bitmask()
        //     & c3.simd_eq(l).to_bitmask()
        //     & d3.simd_eq(p).to_bitmask();
        // let mut mask: u128 = mask0 as u128
        //     | ((mask1 as u128) << 32)
        //     | ((mask2 as u128) << 64)
        //     | ((mask3 as u128) << 96);
        let mut mask = mask0 | (mask1 << 32);
        // let mut mask = mask0 as u32 & ((1 << 29) - 1);
        loop {
            if mask == 0 {
                ptr = ptr.add(64);

                if ptr < end {
                    continue 'solve;
                }

                return sum[0] as u64;
            }

            let idx = mask.trailing_zeros();
            mask &= mask - 1;

            let offset = idx as usize;
            let part = (ptr.add(offset) as *const u8x16).read_unaligned();
            let digits = (part - ascii_zero) & digit_mask;

            // n n n , n n n )
            // 1 2 3 4 5 6 7 8

            let lt: u8x16 = _mm_cmplt_epi8(digits.into(), ten.into()).into();
            // println!("{:?}", digits);
            let m = (_mm_movemask_epi8(lt.into()) as usize & (0b01111111 << 4)) >> 4;

            // const CONTROL: i32 =
            //     _SIDD_UBYTE_OPS | _SIDD_CMP_RANGES | _SIDD_BIT_MASK | _SIDD_LEAST_SIGNIFICANT;
            // let positions: u32x4 =
            //     _mm_cmpestrm::<CONTROL>(range.into(), 2, digits.into(), 16).into();
            // let m = (positions[0] as usize & (0b01111111 << 4)) >> 4;

            // println!(
            //     "{:?} {:032b} {}",
            //     lt,
            //     _mm_movemask_epi8(lt.into()),
            //     positions
            // );
            let shuffled: u8x16 =
                _mm_shuffle_epi8(digits.into(), (*DIGIT_LUT.get_unchecked(m)).into()).into();
            // println!("{:?}", shuffled);

            let test = shuffled & sep_mask;
            let valid: u64x2 = _mm_cmpeq_epi64(test.into(), seps.into()).into();

            // if _mm_testc_si128(valid.into(), valid_mask.into()) == 0 {
            //     continue;
            // }

            // println!(
            //     "{:?} {:?} {:?}",
            //     shuffled,
            //     test,
            //     std::mem::transmute::<u64x2, u8x16>(seps)
            // );

            let digit2: u16x8 = _mm_maddubs_epi16(shuffled.into(), mul2.into()).into();
            let nums: u32x4 = _mm_madd_epi16(digit2.into(), u16x8::splat(1).into()).into();
            // let thing = nums & std::mem::transmute::<u64x2, u32x4>(valid);

            let other: u16x8 = _mm_bsrli_si128::<8>(nums.into()).into();
            // println!("{:?} {:?}", nums, other);
            let finish: u64x2 = _mm_mul_epi32(nums.into(), other.into()).into();

            sum += finish & valid;
        }
    }
}

pub fn part1(s: &str) -> impl Display {
    unsafe { count(s.as_bytes()) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn switch(s: &str) -> u64 {
    loop {}
}

pub fn part2(s: &str) -> impl Display {
    unsafe { switch(s) }
}
