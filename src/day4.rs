use std::hint::black_box;

use super::*;

static mut SAVE: [u8; 512] = [0; 512];

#[export_name = "part1"]
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn scan(s: &[u8]) -> u32 {
    let mut ptr = s.as_ptr();
    let end = s.as_ptr().add(141 * 140);

    // not suspicious at all
    // can't trust compiler to not emit a memcpy call, perform it manually with movsb
    asm!(
        "mov rdi, {dst}",
        "mov rsi, {src}",
        "mov ecx, 512",
        "rep movsb",
        "mov rdi, {src}",
        "mov eax, 'M'",
        "mov ecx, 512",
        "rep stosb",
        dst = in(reg) SAVE.as_ptr(),
        src = in(reg) s.as_ptr().add(141 * 140),
        out("rax") _,
        out("rcx") _,
        out("rdi") _,
        out("rsi") _,
    );

    // black_box(s.as_ptr().add(141 * 140)).copy_to(SAVE.as_mut_ptr(), 512);
    // black_box(s.as_ptr().add(141 * 140) as *mut u8).write_bytes(b'M', 512);

    macro_rules! index {
        ($inc:expr, $row:expr, $off:expr) => {
            (ptr.add($inc).add($row * 141).add($off * 32) as *const u8x32).read_unaligned()
        };
    }

    macro_rules! one {
        ($n:expr) => {
            $n
        };
    }

    macro_rules! two {
        ($n:expr) => {
            $n << 1
        };
    }

    macro_rules! three {
        ($n:expr) => {
            ($n << 1) + $n
        };
    }

    macro_rules! four {
        ($n:expr) => {
            $n << 2
        };
    }

    macro_rules! five {
        ($n:expr) => {
            ($n << 2) + $n
        };
    }

    macro_rules! hash {
        ($a:expr, $b:expr, $c:expr, $d:expr) => {{
            // 1 2 4 3
            // XMAS 0xef
            // SAMX 0x11
            // $a + 2 * $b + 4 * $c + 3 * $d
            // $a + 2 * ($b + 2 * $c + $d) + $d
            // let tmp = _mm256_add_epi8($b.into(), _mm256_add_epi8($c.into(), $c.into()));
            // let tmp = _mm256_add_epi8(tmp, $d.into());
            // let tmp = _mm256_add_epi8($a.into(), _mm256_add_epi8(tmp, tmp));
            // let tmp: u8x32 = _mm256_add_epi8(tmp, $d.into()).into();
            // tmp

            // 1 2 -2 -1
            // XMAS 0x1d
            // SAMX 0xe3
            let tmp = _mm256_add_epi8($a.into(), _mm256_add_epi8($b.into(), $b.into()));
            let tmp = _mm256_sub_epi8(tmp, _mm256_add_epi8($c.into(), $c.into()));
            let tmp = _mm256_sub_epi8(tmp, $d.into());
            let tmp: u8x32 = tmp.into();
            tmp
        }};
    }

    const FORWARD: u8 = 0x1d;
    const BCKWARD: u8 = 0xe3;

    let mut sums: i8x32 = i8x32::splat(0);
    // let mut sums0: i8x32 = Simd::splat(0);
    // let mut sums1: i8x32 = Simd::splat(0);
    // let mut sums2: i8x32 = Simd::splat(0);
    // let mut sums3: i8x32 = Simd::splat(0);

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

        // horizontal
        let hash0 = hash!(r000, r100, r200, r300);
        // vertial
        let hash1 = hash!(r000, r010, r020, r030);
        // top left diagonal
        let hash2 = hash!(r000, r110, r220, r330);
        // top right diagonal
        let hash3 = hash!(r300, r210, r120, r030);

        // horizontal
        let hash4 = hash!(r001, r101, r201, r301);
        // vertial
        let hash5 = hash!(r001, r011, r021, r031);
        // top left diagonal
        let hash6 = hash!(r001, r111, r221, r331);
        // top right diagonal
        let hash7 = hash!(r301, r211, r121, r031);

        // sums0 -= hash0.simd_eq(u8x32::splat(FORWARD)).to_int();
        // sums1 -= hash1.simd_eq(u8x32::splat(FORWARD)).to_int();
        // sums2 -= hash2.simd_eq(u8x32::splat(FORWARD)).to_int();
        // sums3 -= hash3.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash0.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash1.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash2.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash3.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash4.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash5.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash6.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash7.simd_eq(u8x32::splat(FORWARD)).to_int();
        // sums0 -= hash0.simd_eq(u8x32::splat(BCKWARD)).to_int();
        // sums1 -= hash1.simd_eq(u8x32::splat(BCKWARD)).to_int();
        // sums2 -= hash2.simd_eq(u8x32::splat(BCKWARD)).to_int();
        // sums3 -= hash3.simd_eq(u8x32::splat(BCKWARD)).to_int();
        sums -= hash0.simd_eq(u8x32::splat(BCKWARD)).to_int();
        sums -= hash1.simd_eq(u8x32::splat(BCKWARD)).to_int();
        sums -= hash2.simd_eq(u8x32::splat(BCKWARD)).to_int();
        sums -= hash3.simd_eq(u8x32::splat(BCKWARD)).to_int();
        sums -= hash4.simd_eq(u8x32::splat(BCKWARD)).to_int();
        sums -= hash5.simd_eq(u8x32::splat(BCKWARD)).to_int();
        sums -= hash6.simd_eq(u8x32::splat(BCKWARD)).to_int();
        sums -= hash7.simd_eq(u8x32::splat(BCKWARD)).to_int();

        ptr = ptr.add(64);
        if ptr >= end {
            break;
        }
    }

    asm!(
        "mov rdi, {dst}",
        "mov rsi, {src}",
        "mov rcx, 512",
        "rep movsb",
        dst = in(reg) s.as_ptr().add(141 * 140),
        src = in(reg) SAVE.as_ptr(),
        out("rdi") _,
        out("rsi") _,
        out("rcx") _,
    );
    // black_box(s.as_ptr().add(141 * 140) as *mut u8).copy_from(SAVE.as_ptr(), 512);

    // convert to u16 to prevent overflow, while summing
    // let words0: u16x16 = _mm256_maddubs_epi16(sums0.into(), u8x32::splat(1).into()).into();
    // let words1: u16x16 = _mm256_maddubs_epi16(sums1.into(), u8x32::splat(1).into()).into();
    // let words2: u16x16 = _mm256_maddubs_epi16(sums2.into(), u8x32::splat(1).into()).into();
    // let words3: u16x16 = _mm256_maddubs_epi16(sums3.into(), u8x32::splat(1).into()).into();

    // let woords0: u16x16 = _mm256_hadd_epi16(words0.into(), words1.into()).into();
    // let woords1: u16x16 = _mm256_hadd_epi16(words2.into(), words3.into()).into();

    // let wooords0: u32x8 = _mm256_madd_epi16(woords0.into(), u16x16::splat(1).into()).into();
    // let wooords1: u32x8 = _mm256_madd_epi16(woords1.into(), u16x16::splat(1).into()).into();
    // // collect
    // let dwords: u32x8 = _mm256_hadd_epi32(wooords0.into(), wooords1.into()).into();
    // let dwords: u32x8 = _mm256_hadd_epi32(dwords.into(), dwords.into()).into();
    // let dwords: u32x8 = _mm256_hadd_epi32(dwords.into(), dwords.into()).into();
    // println!("dwords = {:?}", dwords);

    let words: u16x16 = _mm256_maddubs_epi16(sums.into(), u8x32::splat(1).into()).into();
    // convert to u32 while summing
    let dwords: u32x8 = _mm256_madd_epi16(words.into(), u16x16::splat(1).into()).into();
    // collect
    let dwords: u32x8 = _mm256_hadd_epi32(dwords.into(), dwords.into()).into();
    let dwords: u32x8 = _mm256_hadd_epi32(dwords.into(), dwords.into()).into();
    return dwords[0] + dwords[4];
}

pub fn part1(s: &str) -> impl std::fmt::Display {
    unsafe { scan(&s.as_bytes()) }
}

pub fn part2(s: &str) -> impl std::fmt::Display {
    0
}
