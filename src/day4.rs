use std::hint::black_box;

use super::*;

static mut SAVE: [u8; 512] = [0; 512];

#[export_name = "part1"]
#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn scan(s: &[u8]) -> u32 {
    let mut ptr = s.as_ptr();
    let end = s.as_ptr().add(141 * 140);

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

    // let x = u8x32::splat(b'X');
    // let m = u8x32::splat(b'M');
    // let a = u8x32::splat(b'A');
    // let s = u8x32::splat(b'S');

    let mut sums: i8x32 = Simd::splat(0);

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
        ($a:expr, $b:expr, $c:expr, $d:expr) => {
            one!($a) + two!($b) + four!($c) + three!($d)
        };
    }

    const FORWARD: u8 = 0xef;
    const BCKWARD: u8 = 0x11;

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
        let hash = hash!(r000, r100, r200, r300);
        sums -= hash.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash.simd_eq(u8x32::splat(BCKWARD)).to_int();

        // vertial
        let hash = hash!(r000, r010, r020, r030);
        sums -= hash.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash.simd_eq(u8x32::splat(BCKWARD)).to_int();

        // top left diagonal
        let hash = hash!(r000, r110, r220, r330);
        sums -= hash.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash.simd_eq(u8x32::splat(BCKWARD)).to_int();

        // top right diagonal
        let hash = hash!(r300, r210, r120, r030);
        sums -= hash.simd_eq(u8x32::splat(FORWARD)).to_int();
        sums -= hash.simd_eq(u8x32::splat(BCKWARD)).to_int();

        ptr = ptr.add(32);
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

    // convert to u16 to prevent overflow
    let words: u16x16 = _mm256_maddubs_epi16(sums.into(), u8x32::splat(1).into()).into();
    let r = words.reduce_sum() as u32;
    return r;
}

pub fn part1(s: &str) -> impl std::fmt::Display {
    // let mut s = String::from(s);
    // s.extend(['M'; 141 * 5]);
    unsafe { scan(&s.as_bytes()[0..140 * 141]) }
}

pub fn part2(s: &str) -> impl std::fmt::Display {
    0
}
