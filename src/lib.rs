#![feature(portable_simd)]
#![feature(avx512_target_feature)]
use std::arch::x86_64::*;
use std::fmt::Display;
use std::simd::prelude::*;

pub mod day3;
