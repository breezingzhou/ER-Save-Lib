use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct RANDOM_APPEAR_EDIT_PARAM_ST {
	pub appearNum: i32,
	pub paramId1: i32,
	pub rate1: i32,
	pub paramId2: i32,
	pub rate2: i32,
	pub paramId3: i32,
	pub rate3: i32,
	pub paramId4: i32,
	pub rate4: i32,
	pub paramId5: i32,
	pub rate5: i32,
	pub paramId6: i32,
	pub rate6: i32,
	pub paramId7: i32,
	pub rate7: i32,
	pub paramId8: i32,
	pub rate8: i32,
	pub paramId9: i32,
	pub rate9: i32,
	pub paramId10: i32,
	pub rate10: i32,
	pub paramId11: i32,
	pub rate11: i32,
	pub paramId12: i32,
	pub rate12: i32,
	pub paramId13: i32,
	pub rate13: i32,
	pub paramId14: i32,
	pub rate14: i32,
	pub paramId15: i32,
	pub rate15: i32,
	pub paramId16: i32,
	pub rate16: i32,
	pub paramId17: i32,
	pub rate17: i32,
	pub paramId18: i32,
	pub rate18: i32,
	pub paramId19: i32,
	pub rate19: i32,
	pub paramId20: i32,
	pub rate20: i32,
	pub paramId21: i32,
	pub rate21: i32,
	pub paramId22: i32,
	pub rate22: i32,
	pub paramId23: i32,
	pub rate23: i32,
	pub paramId24: i32,
	pub rate24: i32,
}