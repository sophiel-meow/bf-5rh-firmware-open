#![no_std]
#![no_main]

pub const MY_SEG: u8 = 2;

#[path = "../../common.rs"]
mod common;

const _: () = common::assert_seg_features();
