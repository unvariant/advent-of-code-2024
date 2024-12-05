use super::*;

#[derive(Clone, Copy)]
#[repr(align(64))]
struct Row {
    x: Bits,
    m: Bits,
    a: Bits,
    s: Bits,
}

#[derive(Clone, Copy)]
struct Bits {
    a: u128,
    b: u64,
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

    let mut rows: [Row; 143] = [Row {
        x: Bits { a: 0, b: 0 },
        m: Bits { a: 0, b: 0 },
        a: Bits { a: 0, b: 0 },
        s: Bits { a: 0, b: 0 },
    }; 143];

    for i in 0..140 {
        let base = ptr as *const u8x32;
        let a = base.add(0).read_unaligned();
        let b = base.add(1).read_unaligned();
        let c = base.add(2).read_unaligned();
        let d = base.add(3).read_unaligned();
        let e = (ptr.add(125) as *const u8x16).read_unaligned();

        rows[i].x.a = (a.simd_eq(x32).to_bitmask() as u128)
            | ((b.simd_eq(x32).to_bitmask() as u128) << 32)
            | ((c.simd_eq(x32).to_bitmask() as u128) << 64)
            | ((d.simd_eq(x32).to_bitmask() as u128) << 96);
        rows[i].x.b = e.simd_eq(x16).to_bitmask();

        rows[i].m.a = (a.simd_eq(m32).to_bitmask() as u128)
            | ((b.simd_eq(m32).to_bitmask() as u128) << 32)
            | ((c.simd_eq(m32).to_bitmask() as u128) << 64)
            | ((d.simd_eq(m32).to_bitmask() as u128) << 96);
        rows[i].m.b = e.simd_eq(m16).to_bitmask();

        rows[i].a.a = (a.simd_eq(a32).to_bitmask() as u128)
            | ((b.simd_eq(a32).to_bitmask() as u128) << 32)
            | ((c.simd_eq(a32).to_bitmask() as u128) << 64)
            | ((d.simd_eq(a32).to_bitmask() as u128) << 96);
        rows[i].a.b = e.simd_eq(a16).to_bitmask();

        rows[i].s.a = (a.simd_eq(s32).to_bitmask() as u128)
            | ((b.simd_eq(s32).to_bitmask() as u128) << 32)
            | ((c.simd_eq(s32).to_bitmask() as u128) << 64)
            | ((d.simd_eq(s32).to_bitmask() as u128) << 96);
        rows[i].s.b = e.simd_eq(s16).to_bitmask();

        ptr = ptr.add(141);
    }

    for i in 0..140 {
        let horizontal_forward =
            rows[i].x.a & (rows[i].m.a << 1) & (rows[i].a.a << 2) & (rows[i].s.a << 3);
        let horizontal_backward =
            rows[i].s.a & (rows[i].a.a << 1) & (rows[i].m.a << 2) & (rows[i].x.a << 3);

        let vertical_forward = rows[i].x.a & rows[i + 1].m.a & rows[i + 2].a.a & rows[i + 3].s.a;
        let vertical_backward = rows[i].s.a & rows[i + 1].a.a & rows[i + 2].m.a & rows[i + 3].x.a;

        let top_left_forward =
            rows[i].x.a & (rows[i + 1].m.a >> 1) & (rows[i + 2].a.a >> 2) & (rows[i + 3].s.a >> 3);
        let top_left_backward =
            rows[i].s.a & (rows[i + 1].a.a >> 1) & (rows[i + 2].m.a >> 2) & (rows[i + 3].x.a >> 3);

        let top_right_forward =
            rows[i].x.a & (rows[i + 1].m.a << 1) & (rows[i + 2].a.a << 2) & (rows[i + 3].s.a << 3);
        let top_right_backward =
            rows[i].s.a & (rows[i + 1].a.a << 1) & (rows[i + 2].m.a << 2) & (rows[i + 3].x.a << 3);

        total += horizontal_forward.count_ones()
            + horizontal_backward.count_ones()
            + vertical_forward.count_ones()
            + vertical_backward.count_ones()
            + top_left_forward.count_ones()
            + top_left_backward.count_ones()
            + top_right_forward.count_ones()
            + top_right_backward.count_ones();

        const MASK: u64 = 0xffffffffffffffffu64 & !0b111u64;

        let horizontal_forward =
            rows[i].x.b & (rows[i].m.b << 1) & (rows[i].a.b << 2) & (rows[i].s.b << 3) & MASK;
        let horizontal_backward =
            rows[i].s.b & (rows[i].a.b << 1) & (rows[i].m.b << 2) & (rows[i].x.b << 3) & MASK;

        let vertical_forward =
            rows[i].x.b & rows[i + 1].m.b & rows[i + 2].a.b & rows[i + 3].s.b & MASK;
        let vertical_backward =
            rows[i].s.b & rows[i + 1].a.b & rows[i + 2].m.b & rows[i + 3].x.b & MASK;

        let top_left_forward =
            rows[i].x.b & (rows[i + 1].m.b >> 1) & (rows[i + 2].a.b >> 2) & (rows[i + 3].s.b >> 3);
        let top_left_backward =
            rows[i].s.b & (rows[i + 1].a.b >> 1) & (rows[i + 2].m.b >> 2) & (rows[i + 3].x.b >> 3);
        let top_right_forward =
            rows[i].x.b & (rows[i + 1].m.b << 1) & (rows[i + 2].a.b << 2) & (rows[i + 3].s.b << 3);
        let top_right_backward =
            rows[i].s.b & (rows[i + 1].a.b << 1) & (rows[i + 2].m.b << 2) & (rows[i + 3].x.b << 3);

        total += horizontal_forward.count_ones()
            + horizontal_backward.count_ones()
            + vertical_forward.count_ones()
            + vertical_backward.count_ones()
            + top_left_forward.count_ones()
            + top_left_backward.count_ones()
            + top_right_forward.count_ones()
            + top_right_backward.count_ones();
    }

    total
}

pub fn part1(s: &str) -> impl std::fmt::Display {
    unsafe { scan(s.as_bytes()) }
}

pub fn part2(s: &str) -> impl std::fmt::Display {
    0
}
