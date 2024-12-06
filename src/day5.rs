use core::slice;
use std::mem::transmute;

use super::*;

#[repr(align(0x1000))]
struct Rules([[u32; 32]; 100]);

static mut RULES: Rules = Rules([[0; 32]; 100]);
static mut PTRS: [*mut u32; 100] = unsafe {
    [
        &mut RULES.0[0] as *mut u32,
        &mut RULES.0[1] as *mut u32,
        &mut RULES.0[2] as *mut u32,
        &mut RULES.0[3] as *mut u32,
        &mut RULES.0[4] as *mut u32,
        &mut RULES.0[5] as *mut u32,
        &mut RULES.0[6] as *mut u32,
        &mut RULES.0[7] as *mut u32,
        &mut RULES.0[8] as *mut u32,
        &mut RULES.0[9] as *mut u32,
        &mut RULES.0[10] as *mut u32,
        &mut RULES.0[11] as *mut u32,
        &mut RULES.0[12] as *mut u32,
        &mut RULES.0[13] as *mut u32,
        &mut RULES.0[14] as *mut u32,
        &mut RULES.0[15] as *mut u32,
        &mut RULES.0[16] as *mut u32,
        &mut RULES.0[17] as *mut u32,
        &mut RULES.0[18] as *mut u32,
        &mut RULES.0[19] as *mut u32,
        &mut RULES.0[20] as *mut u32,
        &mut RULES.0[21] as *mut u32,
        &mut RULES.0[22] as *mut u32,
        &mut RULES.0[23] as *mut u32,
        &mut RULES.0[24] as *mut u32,
        &mut RULES.0[25] as *mut u32,
        &mut RULES.0[26] as *mut u32,
        &mut RULES.0[27] as *mut u32,
        &mut RULES.0[28] as *mut u32,
        &mut RULES.0[29] as *mut u32,
        &mut RULES.0[30] as *mut u32,
        &mut RULES.0[31] as *mut u32,
        &mut RULES.0[32] as *mut u32,
        &mut RULES.0[33] as *mut u32,
        &mut RULES.0[34] as *mut u32,
        &mut RULES.0[35] as *mut u32,
        &mut RULES.0[36] as *mut u32,
        &mut RULES.0[37] as *mut u32,
        &mut RULES.0[38] as *mut u32,
        &mut RULES.0[39] as *mut u32,
        &mut RULES.0[40] as *mut u32,
        &mut RULES.0[41] as *mut u32,
        &mut RULES.0[42] as *mut u32,
        &mut RULES.0[43] as *mut u32,
        &mut RULES.0[44] as *mut u32,
        &mut RULES.0[45] as *mut u32,
        &mut RULES.0[46] as *mut u32,
        &mut RULES.0[47] as *mut u32,
        &mut RULES.0[48] as *mut u32,
        &mut RULES.0[49] as *mut u32,
        &mut RULES.0[50] as *mut u32,
        &mut RULES.0[51] as *mut u32,
        &mut RULES.0[52] as *mut u32,
        &mut RULES.0[53] as *mut u32,
        &mut RULES.0[54] as *mut u32,
        &mut RULES.0[55] as *mut u32,
        &mut RULES.0[56] as *mut u32,
        &mut RULES.0[57] as *mut u32,
        &mut RULES.0[58] as *mut u32,
        &mut RULES.0[59] as *mut u32,
        &mut RULES.0[60] as *mut u32,
        &mut RULES.0[61] as *mut u32,
        &mut RULES.0[62] as *mut u32,
        &mut RULES.0[63] as *mut u32,
        &mut RULES.0[64] as *mut u32,
        &mut RULES.0[65] as *mut u32,
        &mut RULES.0[66] as *mut u32,
        &mut RULES.0[67] as *mut u32,
        &mut RULES.0[68] as *mut u32,
        &mut RULES.0[69] as *mut u32,
        &mut RULES.0[70] as *mut u32,
        &mut RULES.0[71] as *mut u32,
        &mut RULES.0[72] as *mut u32,
        &mut RULES.0[73] as *mut u32,
        &mut RULES.0[74] as *mut u32,
        &mut RULES.0[75] as *mut u32,
        &mut RULES.0[76] as *mut u32,
        &mut RULES.0[77] as *mut u32,
        &mut RULES.0[78] as *mut u32,
        &mut RULES.0[79] as *mut u32,
        &mut RULES.0[80] as *mut u32,
        &mut RULES.0[81] as *mut u32,
        &mut RULES.0[82] as *mut u32,
        &mut RULES.0[83] as *mut u32,
        &mut RULES.0[84] as *mut u32,
        &mut RULES.0[85] as *mut u32,
        &mut RULES.0[86] as *mut u32,
        &mut RULES.0[87] as *mut u32,
        &mut RULES.0[88] as *mut u32,
        &mut RULES.0[89] as *mut u32,
        &mut RULES.0[90] as *mut u32,
        &mut RULES.0[91] as *mut u32,
        &mut RULES.0[92] as *mut u32,
        &mut RULES.0[93] as *mut u32,
        &mut RULES.0[94] as *mut u32,
        &mut RULES.0[95] as *mut u32,
        &mut RULES.0[96] as *mut u32,
        &mut RULES.0[97] as *mut u32,
        &mut RULES.0[98] as *mut u32,
        &mut RULES.0[99] as *mut u32,
    ]
};
static mut SCRATCH: [u32; 512] = [0; 512];
static mut INDEXES: [u32; 100 * 512] = [0x7fffffff; 100 * 512];
const MASKS: [u8x32; 33] = [
    u8x32::from_array([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]),
    u8x32::from_array([
        255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]),
    u8x32::from_array([
        255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0,
    ]),
    u8x32::from_array([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ]),
];

#[export_name = "part1"]
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn magic_the_gathering(s: &[u8]) -> u32 {
    let start = s.as_ptr().sub(1);
    let rules_shuffle = u8x32::from_array([
        0x01, 0x02, 0x04, 0x05, 0x07, 0x08, 0x0a, 0x0b, 0x0d, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, /* lower 128 bit */
        0x00, 0x01, 0x03, 0x04, 0x06, 0x07, 0x09, 0x0a, 0x0c, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, /* upper 128 bit */
    ]);
    let mul: u8x32 = transmute(u16x16::splat(10 | (1 << 8)));
    let ascii_zero = u8x32::splat(0x30);

    // |NN| NN|N N|NN |NN|
    // NN|N N|NN |NN| NN|N
    // 30 bytes per iteration

    const ITERATIONS: usize = 6 * 1176 / 30;
    for i in 0..ITERATIONS {
        let chunk = (start.add(30 * i) as *const u8x32).read_unaligned();
        let digits = chunk - ascii_zero;
        let nums: u16x16 = _mm256_maddubs_epi16(
            _mm256_shuffle_epi8(digits.into(), rules_shuffle.into()),
            mul.into(),
        )
        .into();

        for i in 0..2 {
            PTRS[nums[i * 2] as usize].write(nums[i * 2 + 1] as u32);
            PTRS[nums[i * 2] as usize] = PTRS[nums[i * 2] as usize].add(1);
            PTRS[nums[9 + i * 2] as usize].write(nums[9 + i * 2 + 1] as u32);
            PTRS[nums[9 + i * 2] as usize] = PTRS[nums[9 + i * 2] as usize].add(1);
        }

        PTRS[nums[4] as usize].write(nums[8] as u32);
        PTRS[nums[4] as usize] = PTRS[nums[4] as usize].add(1);
    }

    const LEFTOVER: i32 = (6 * 1176 - 6 * 1176 / 30 * 30) / 6;
    let mut start = s.as_ptr().add(6 * 1176 / 30 * 30);
    for _ in 0..LEFTOVER {
        let a = start.add(0).read() as u32 * 10 + start.add(1).read() as u32 - (0x30 * 10 + 0x30);
        let b = start.add(3).read() as u32 * 10 + start.add(4).read() as u32 - (0x30 * 10 + 0x30);
        PTRS[a as usize].write(b);
        PTRS[a as usize] = PTRS[a as usize].add(1);
        start = start.add(6);
    }

    let mut start = s.as_ptr().add(1176 * 6 + 1 - 1);
    let end = s.as_ptr().add(s.len());
    let newline = u8x32::splat(0x0a);
    let mut ptr = SCRATCH.as_mut_ptr() as *mut u8;
    let mut indexes = INDEXES.as_mut_ptr();
    let mut count = 0;
    let mut idx = 0;
    let mut ans = 0;

    loop {
        let chunk = (start as *const u8x32).read_unaligned();
        let pos = (chunk.simd_eq(newline).to_bitmask() as u32 & !1).trailing_zeros();
        let digits = (chunk - ascii_zero) & MASKS[pos as usize];
        let nums: u16x16 = _mm256_maddubs_epi16(
            _mm256_shuffle_epi8(digits.into(), rules_shuffle.into()),
            mul.into(),
        )
        .into();

        (ptr as *mut u16x16).write_unaligned(nums);
        (ptr.add(10) as *mut u8x16)
            .write_unaligned(_mm256_extractf128_si256::<1>(nums.into()).into());

        if pos != 32 {
            let mut valid = true;
            let n = (count * 10 + (pos + 1) / 3) as usize;
            let pages = SCRATCH.as_ptr() as *const u16;
            // println!("{:?}", slice::from_raw_parts(pages, n as usize));
            for i in 0..n {
                indexes
                    .add(pages.add(i as usize).read() as usize)
                    .write(i as u32);
            }

            // println!("[{idx}]");
            for i in 0..n {
                let page = pages.add(i as usize).read();
                let curr = RULES.0[page as usize].as_ptr() as *const u32x8;
                let index = i32x8::splat(i as i32 - 1);
                let a: u32x8 = _mm256_i32gather_epi32::<4>(
                    indexes as *const i32,
                    curr.add(0).read_unaligned().into(),
                )
                .into();
                let b: u32x8 = _mm256_i32gather_epi32::<4>(
                    indexes as *const i32,
                    curr.add(1).read_unaligned().into(),
                )
                .into();
                let c: u32x8 = _mm256_i32gather_epi32::<4>(
                    indexes as *const i32,
                    curr.add(2).read_unaligned().into(),
                )
                .into();

                let a: u32x8 = _mm256_cmpgt_epi32(index.into(), a.into()).into();
                let b: u32x8 = _mm256_cmpgt_epi32(index.into(), b.into()).into();
                let c: u32x8 = _mm256_cmpgt_epi32(index.into(), c.into()).into();

                let is_ordered = _mm256_movemask_ps(transmute(a | b | c)) == 0;
                // if i == 0 {
                // println!("{page} rules {a:?} {b:?} {c:?}");
                // println!("{is_ordered}");
                // }
                valid = valid && is_ordered;
            }

            // println!("{idx} is valid: {valid}");
            ans += pages.add(n / 2).read() * valid as u16;

            count = 0;
            indexes = indexes.add(100);
            start = start.add(pos as usize);
            ptr = SCRATCH.as_mut_ptr() as *mut u8;
            idx += 1;
            if start >= end.sub(1) {
                break;
            }
        } else {
            start = start.add(30);
            ptr = ptr.add(20);
            count = count + 1;
        }
    }

    ans as u32
}

pub fn part1(s: &str) -> impl Display {
    unsafe { magic_the_gathering(s.as_bytes()) }
}

pub fn part2(s: &str) -> impl Display {
    0
}
