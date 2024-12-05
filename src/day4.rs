use super::*;

#[export_name = "part1"]
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn scan(s: &[u8]) -> u32 {
    let mut total = 0;
    let mut ptr = s.as_ptr();
    let end = s.as_ptr().add(s.len());

    let x = u8x32::splat(b'X');
    let m = u8x32::splat(b'M');
    let a = u8x32::splat(b'A');
    let s = u8x32::splat(b'S');

    macro_rules! index {
        ($inc:expr, $row:expr, $off:expr) => {
            (ptr.add($inc).add($row * 141).add($off * 32) as *const u8x32).read_unaligned()
        };
    }

    loop {
        let r000 = index!(0, 0, 0);
        let r100 = index!(1, 0, 0);
        let r200 = index!(2, 0, 0);
        let r300 = index!(3, 0, 0);

        let r010 = index!(0, 1, 0);
        let r020 = index!(0, 2, 0);
        let r030 = index!(0, 3, 0);

        let r120 = index!(1, 2, 0);
        let r210 = index!(2, 1, 0);

        let r110 = index!(1, 1, 0);
        let r220 = index!(2, 2, 0);
        let r330 = index!(3, 3, 0);

        let r001 = index!(0, 0, 1);
        let r101 = index!(1, 0, 1);
        let r201 = index!(2, 0, 1);
        let r301 = index!(3, 0, 1);

        let r011 = index!(0, 1, 1);
        let r021 = index!(0, 2, 1);
        let r031 = index!(0, 3, 1);

        let r121 = index!(1, 2, 1);
        let r211 = index!(2, 1, 1);

        let r111 = index!(1, 1, 1);
        let r221 = index!(2, 2, 1);
        let r331 = index!(3, 3, 1);

        let horizontal_forward =
            (r000.simd_eq(x) & r010.simd_eq(m) & r020.simd_eq(a) & r030.simd_eq(s)).to_bitmask()
                | ((r001.simd_eq(x) & r011.simd_eq(m) & r021.simd_eq(a) & r031.simd_eq(s))
                    .to_bitmask()
                    << 32);
        let horizontal_bckward =
            (r000.simd_eq(s) & r010.simd_eq(a) & r020.simd_eq(m) & r030.simd_eq(x)).to_bitmask()
                | ((r001.simd_eq(s) & r011.simd_eq(a) & r021.simd_eq(m) & r031.simd_eq(x))
                    .to_bitmask()
                    << 32);
        let vertical_forward =
            (r000.simd_eq(x) & r100.simd_eq(m) & r200.simd_eq(a) & r300.simd_eq(s)).to_bitmask()
                | ((r001.simd_eq(x) & r101.simd_eq(m) & r201.simd_eq(a) & r301.simd_eq(s))
                    .to_bitmask()
                    << 32);
        let vertical_bckward =
            (r000.simd_eq(s) & r100.simd_eq(a) & r200.simd_eq(m) & r300.simd_eq(x)).to_bitmask()
                | ((r001.simd_eq(s) & r101.simd_eq(a) & r201.simd_eq(m) & r301.simd_eq(x))
                    .to_bitmask()
                    << 32);
        let top_right_forward =
            (r030.simd_eq(x) & r120.simd_eq(m) & r210.simd_eq(a) & r300.simd_eq(s)).to_bitmask()
                | ((r031.simd_eq(x) & r121.simd_eq(m) & r211.simd_eq(a) & r301.simd_eq(s))
                    .to_bitmask()
                    << 32);
        let top_right_bckward =
            (r030.simd_eq(s) & r120.simd_eq(a) & r210.simd_eq(m) & r300.simd_eq(x)).to_bitmask()
                | ((r031.simd_eq(s) & r121.simd_eq(a) & r211.simd_eq(m) & r301.simd_eq(x))
                    .to_bitmask()
                    << 32);
        let top_left_forward =
            (r000.simd_eq(x) & r110.simd_eq(m) & r220.simd_eq(a) & r330.simd_eq(s)).to_bitmask()
                | ((r001.simd_eq(x) & r111.simd_eq(m) & r221.simd_eq(a) & r331.simd_eq(s))
                    .to_bitmask()
                    << 32);
        let top_left_bckward =
            (r000.simd_eq(s) & r110.simd_eq(a) & r220.simd_eq(m) & r330.simd_eq(x)).to_bitmask()
                | ((r001.simd_eq(s) & r111.simd_eq(a) & r221.simd_eq(m) & r331.simd_eq(x))
                    .to_bitmask()
                    << 32);

        total += horizontal_forward.count_ones()
            + horizontal_bckward.count_ones()
            + vertical_forward.count_ones()
            + vertical_bckward.count_ones()
            + top_left_forward.count_ones()
            + top_left_bckward.count_ones()
            + top_right_forward.count_ones()
            + top_right_bckward.count_ones();

        ptr = ptr.add(64);
        if ptr >= end {
            return total;
        }
    }
}

pub fn part1(s: &str) -> impl std::fmt::Display {
    unsafe { scan(s.as_bytes()) }
}

pub fn part2(s: &str) -> impl std::fmt::Display {
    0
}
