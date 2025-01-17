use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_AA_QUALITY_DETAIL {
	pub enabled: u8,
	pub forceFXAA2: u8,
	pub dmy: [u8;2],
}
