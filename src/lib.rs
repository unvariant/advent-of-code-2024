#![feature(portable_simd)]
#![feature(avx512_target_feature)]
use std::arch::asm;
use std::arch::global_asm;
use std::arch::x86_64::*;
use std::fmt::Display;
use std::simd::prelude::*;

pub mod day3;
pub mod day4;
