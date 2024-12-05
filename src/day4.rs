use super::*;

struct Row32 {
    x: u128,
    m: u128,
    a: u128,
    s: u128,
}

macro_rules! row32 {
    ($expression:expr, $x:expr, $m:expr, $a:expr, $s:expr) => {{
        let curr = $expression as *const u8x32;

        let r0 = curr.add(0).read_unaligned();
        let r1 = curr.add(1).read_unaligned();
        let r2 = curr.add(2).read_unaligned();
        let r3 = curr.add(3).read_unaligned();

        Row32 {
            x: (r0.simd_eq($x).to_bitmask() as u128
                | ((r1.simd_eq($x).to_bitmask() as u128) << 32)
                | ((r2.simd_eq($x).to_bitmask() as u128) << 64)
                | ((r3.simd_eq($x).to_bitmask() as u128) << 96)),
            m: r0.simd_eq($m).to_bitmask() as u128
                | ((r1.simd_eq($m).to_bitmask() as u128) << 32)
                | ((r2.simd_eq($m).to_bitmask() as u128) << 64)
                | ((r3.simd_eq($m).to_bitmask() as u128) << 96),
            a: r0.simd_eq($a).to_bitmask() as u128
                | ((r1.simd_eq($a).to_bitmask() as u128) << 32)
                | ((r2.simd_eq($a).to_bitmask() as u128) << 64)
                | ((r3.simd_eq($a).to_bitmask() as u128) << 96),
            s: r0.simd_eq($s).to_bitmask() as u128
                | ((r1.simd_eq($s).to_bitmask() as u128) << 32)
                | ((r2.simd_eq($s).to_bitmask() as u128) << 64)
                | ((r3.simd_eq($s).to_bitmask() as u128) << 96),
        }
    }};
}

struct Row16 {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn row16(base: *const u8) -> Row16 {
    let x = u8x16::splat(b'X');
    let m = u8x16::splat(b'M');
    let a = u8x16::splat(b'A');
    let s = u8x16::splat(b'S');

    let curr = base as *const u8x16;

    let r0 = curr.add(0).read_unaligned();

    Row16 {
        x: r0.simd_eq(x).to_bitmask(),
        m: r0.simd_eq(m).to_bitmask(),
        a: r0.simd_eq(a).to_bitmask(),
        s: r0.simd_eq(s).to_bitmask(),
    }
}

macro_rules! row16 {
    ($expression:expr, $x:expr, $m:expr, $a:expr, $s:expr) => {{
        let curr = $expression as *const u8x16;

        let r0 = curr.add(0).read_unaligned();

        Row16 {
            x: r0.simd_eq($x).to_bitmask(),
            m: r0.simd_eq($m).to_bitmask(),
            a: r0.simd_eq($a).to_bitmask(),
            s: r0.simd_eq($s).to_bitmask(),
        }
    }};
}

#[export_name = "part1"]
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn scan(s: &[u8]) -> u32 {
    let mut sums: [u32; 8] = [0; 8];
    let mut ptr = s.as_ptr();

    let x32 = u8x32::splat(b'X');
    let m32 = u8x32::splat(b'M');
    let a32 = u8x32::splat(b'A');
    let s32 = u8x32::splat(b'S');

    let x16 = u8x16::splat(b'X');
    let m16 = u8x16::splat(b'M');
    let a16 = u8x16::splat(b'A');
    let s16 = u8x16::splat(b'S');

    let mut row0x32;
    let mut row1x32 = row32!(ptr.add(141 * 0), x32, m32, a32, s32);
    let mut row2x32 = row32!(ptr.add(141 * 1), x32, m32, a32, s32);
    let mut row3x32 = row32!(ptr.add(141 * 2), x32, m32, a32, s32);

    let mut row0x16;
    let mut row1x16 = row16!(ptr.add(141 * 0 + 125), x16, m16, a16, s16);
    let mut row2x16 = row16!(ptr.add(141 * 1 + 125), x16, m16, a16, s16);
    let mut row3x16 = row16!(ptr.add(141 * 2 + 125), x16, m16, a16, s16);

    for _ in 0..140 {
        let next = ptr.add(141);
        row0x32 = row1x32;
        row1x32 = row2x32;
        row2x32 = row3x32;
        row3x32 = row32!(ptr.add(141 * 3), x32, m32, a32, s32);

        row0x16 = row1x16;
        row1x16 = row2x16;
        row2x16 = row3x16;
        row3x16 = row16!(ptr.add(141 * 3 + 125), x16, m16, a16, s16);

        let horizontal_forward = row0x32.x & (row0x32.m << 1) & (row0x32.a << 2) & (row0x32.s << 3);
        let horizontal_backward =
            row0x32.s & (row0x32.a << 1) & (row0x32.m << 2) & (row0x32.x << 3);

        let vertical_forward = row0x32.x & row1x32.m & row2x32.a & row3x32.s;
        let vertical_backward = row0x32.s & row1x32.a & row2x32.m & row3x32.x;

        let top_left_forward = row0x32.x & (row1x32.m >> 1) & (row2x32.a >> 2) & (row3x32.s >> 3);
        let top_left_backward = row0x32.s & (row1x32.a >> 1) & (row2x32.m >> 2) & (row3x32.x >> 3);

        let top_right_forward = row0x32.x & (row1x32.m << 1) & (row2x32.a << 2) & (row3x32.s << 3);
        let top_right_backward = row0x32.s & (row1x32.a << 1) & (row2x32.m << 2) & (row3x32.x << 3);

        sums[0] += horizontal_forward.count_ones();
        sums[1] += horizontal_backward.count_ones();
        sums[2] += vertical_forward.count_ones();
        sums[3] += vertical_backward.count_ones();
        sums[4] += top_left_forward.count_ones();
        sums[5] += top_left_backward.count_ones();
        sums[6] += top_right_forward.count_ones();
        sums[7] += top_right_backward.count_ones();

        const MASK: u64 = 0xffffffffffffffffu64 & !0b111u64;

        let horizontal_forward =
            row0x16.x & (row0x16.m << 1) & (row0x16.a << 2) & (row0x16.s << 3) & MASK;
        let horizontal_backward =
            row0x16.s & (row0x16.a << 1) & (row0x16.m << 2) & (row0x16.x << 3) & MASK;

        let vertical_forward = row0x16.x & row1x16.m & row2x16.a & row3x16.s & MASK;
        let vertical_backward = row0x16.s & row1x16.a & row2x16.m & row3x16.x & MASK;

        let top_left_forward = row0x16.x & (row1x16.m >> 1) & (row2x16.a >> 2) & (row3x16.s >> 3);
        let top_left_backward = row0x16.s & (row1x16.a >> 1) & (row2x16.m >> 2) & (row3x16.x >> 3);

        let top_right_forward = row0x16.x & (row1x16.m << 1) & (row2x16.a << 2) & (row3x16.s << 3);
        let top_right_backward = row0x16.s & (row1x16.a << 1) & (row2x16.m << 2) & (row3x16.x << 3);

        sums[0] += horizontal_forward.count_ones();
        sums[1] += horizontal_backward.count_ones();
        sums[2] += vertical_forward.count_ones();
        sums[3] += vertical_backward.count_ones();
        sums[4] += top_left_forward.count_ones();
        sums[5] += top_left_backward.count_ones();
        sums[6] += top_right_forward.count_ones();
        sums[7] += top_right_backward.count_ones();
        ptr = next;
        //
    }

    sums[0] + sums[1] + sums[2] + sums[3] + sums[4] + sums[5] + sums[6] + sums[7]
}

pub fn part1(s: &str) -> impl std::fmt::Display {
    unsafe { scan(s.as_bytes()) }
}

pub fn part2(s: &str) -> impl std::fmt::Display {
    0
}
