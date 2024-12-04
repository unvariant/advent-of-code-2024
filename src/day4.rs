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
    let mut total = 0;
    let mut ptr = s.as_ptr();

    let x32 = u8x32::splat(b'X');
    let m32 = u8x32::splat(b'M');
    let a32 = u8x32::splat(b'A');
    let s32 = u8x32::splat(b'S');

    let x16 = u8x16::splat(b'X');
    let m16 = u8x16::splat(b'M');
    let a16 = u8x16::splat(b'A');
    let s16 = u8x16::splat(b'S');

    for _ in 0..140 {
        let next = ptr.add(141);

        let row0 = row32!(ptr.add(141 * 0), x32, m32, a32, s32);
        let row1 = row32!(ptr.add(141 * 1), x32, m32, a32, s32);
        let row2 = row32!(ptr.add(141 * 2), x32, m32, a32, s32);
        let row3 = row32!(ptr.add(141 * 3), x32, m32, a32, s32);

        let horizontal_forward = row0.x & (row0.m << 1) & (row0.a << 2) & (row0.s << 3);
        let horizontal_backward = row0.s & (row0.a << 1) & (row0.m << 2) & (row0.x << 3);

        let vertical_forward = row0.x & row1.m & row2.a & row3.s;
        let vertical_backward = row0.s & row1.a & row2.m & row3.x;

        let top_left_forward = row0.x & (row1.m >> 1) & (row2.a >> 2) & (row3.s >> 3);
        let top_left_backward = row0.s & (row1.a >> 1) & (row2.m >> 2) & (row3.x >> 3);

        let top_right_forward = row0.x & (row1.m << 1) & (row2.a << 2) & (row3.s << 3);
        let top_right_backward = row0.s & (row1.a << 1) & (row2.m << 2) & (row3.x << 3);

        total += horizontal_forward.count_ones()
            + horizontal_backward.count_ones()
            + vertical_forward.count_ones()
            + vertical_backward.count_ones()
            + top_left_forward.count_ones()
            + top_left_backward.count_ones()
            + top_right_forward.count_ones()
            + top_right_backward.count_ones();

        let row0 = row16!(ptr.add(141 * 0 + 125), x16, m16, a16, s16);
        let row1 = row16!(ptr.add(141 * 1 + 125), x16, m16, a16, s16);
        let row2 = row16!(ptr.add(141 * 2 + 125), x16, m16, a16, s16);
        let row3 = row16!(ptr.add(141 * 3 + 125), x16, m16, a16, s16);

        const MASK: u64 = 0xffffffffffffffffu64 & !0b111u64;

        let horizontal_forward = row0.x & (row0.m << 1) & (row0.a << 2) & (row0.s << 3) & MASK;
        let horizontal_backward = row0.s & (row0.a << 1) & (row0.m << 2) & (row0.x << 3) & MASK;

        let vertical_forward = row0.x & row1.m & row2.a & row3.s & MASK;
        let vertical_backward = row0.s & row1.a & row2.m & row3.x & MASK;

        let top_left_forward = row0.x & (row1.m >> 1) & (row2.a >> 2) & (row3.s >> 3);
        let top_left_backward = row0.s & (row1.a >> 1) & (row2.m >> 2) & (row3.x >> 3);

        let top_right_forward = row0.x & (row1.m << 1) & (row2.a << 2) & (row3.s << 3);
        let top_right_backward = row0.s & (row1.a << 1) & (row2.m << 2) & (row3.x << 3);

        total += horizontal_forward.count_ones()
            + horizontal_backward.count_ones()
            + vertical_forward.count_ones()
            + vertical_backward.count_ones()
            + top_left_forward.count_ones()
            + top_left_backward.count_ones()
            + top_right_forward.count_ones()
            + top_right_backward.count_ones();

        ptr = next;
    }

    total
}

pub fn part1(s: &str) -> impl std::fmt::Display {
    unsafe { scan(s.as_bytes()) }
}

pub fn part2(s: &str) -> impl std::fmt::Display {
    0
}
