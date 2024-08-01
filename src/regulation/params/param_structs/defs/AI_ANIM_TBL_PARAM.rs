use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct AI_ANIM_TBL_PARAM {
	pub atk0_EzStateId: i16,
	pub atk1_EzStateId: i16,
	pub atk2_EzStateId: i16,
	pub atk3_EzStateId: i16,
	pub atk4_EzStateId: i16,
	pub atk5_EzStateId: i16,
	pub atk6_EzStateId: i16,
	pub atk7_EzStateId: i16,
	pub atk8_EzStateId: i16,
	pub atk9_EzStateId: i16,
	pub atk10_EzStateId: i16,
	pub atk11_EzStateId: i16,
	pub atk12_EzStateId: i16,
	pub atk13_EzStateId: i16,
	pub atk14_EzStateId: i16,
	pub atk15_EzStateId: i16,
	pub atk16_EzStateId: i16,
	pub atk17_EzStateId: i16,
	pub atk18_EzStateId: i16,
	pub atk19_EzStateId: i16,
	pub atk20_EzStateId: i16,
	pub atk21_EzStateId: i16,
	pub atk22_EzStateId: i16,
	pub atk23_EzStateId: i16,
	pub atk24_EzStateId: i16,
	pub atk25_EzStateId: i16,
	pub atk26_EzStateId: i16,
	pub atk27_EzStateId: i16,
	pub atk28_EzStateId: i16,
	pub atk29_EzStateId: i16,
	pub atk0_MinDist: i16,
	pub atk1_MinDist: i16,
	pub atk2_MinDist: i16,
	pub atk3_MinDist: i16,
	pub atk4_MinDist: i16,
	pub atk5_MinDist: i16,
	pub atk6_MinDist: i16,
	pub atk7_MinDist: i16,
	pub atk8_MinDist: i16,
	pub atk9_MinDist: i16,
	pub atk10_MinDist: i16,
	pub atk11_MinDist: i16,
	pub atk12_MinDist: i16,
	pub atk13_MinDist: i16,
	pub atk14_MinDist: i16,
	pub atk15_MinDist: i16,
	pub atk16_MinDist: i16,
	pub atk17_MinDist: i16,
	pub atk18_MinDist: i16,
	pub atk19_MinDist: i16,
	pub atk20_MinDist: i16,
	pub atk21_MinDist: i16,
	pub atk22_MinDist: i16,
	pub atk23_MinDist: i16,
	pub atk24_MinDist: i16,
	pub atk25_MinDist: i16,
	pub atk26_MinDist: i16,
	pub atk27_MinDist: i16,
	pub atk28_MinDist: i16,
	pub atk29_MinDist: i16,
	pub atk0_MaxDist: i16,
	pub atk1_MaxDist: i16,
	pub atk2_MaxDist: i16,
	pub atk3_MaxDist: i16,
	pub atk4_MaxDist: i16,
	pub atk5_MaxDist: i16,
	pub atk6_MaxDist: i16,
	pub atk7_MaxDist: i16,
	pub atk8_MaxDist: i16,
	pub atk9_MaxDist: i16,
	pub atk10_MaxDist: i16,
	pub atk11_MaxDist: i16,
	pub atk12_MaxDist: i16,
	pub atk13_MaxDist: i16,
	pub atk14_MaxDist: i16,
	pub atk15_MaxDist: i16,
	pub atk16_MaxDist: i16,
	pub atk17_MaxDist: i16,
	pub atk18_MaxDist: i16,
	pub atk19_MaxDist: i16,
	pub atk20_MaxDist: i16,
	pub atk21_MaxDist: i16,
	pub atk22_MaxDist: i16,
	pub atk23_MaxDist: i16,
	pub atk24_MaxDist: i16,
	pub atk25_MaxDist: i16,
	pub atk26_MaxDist: i16,
	pub atk27_MaxDist: i16,
	pub atk28_MaxDist: i16,
	pub atk29_MaxDist: i16,
	#[deku(bits = 4)]
	pub atk0_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk1_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk2_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk3_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk4_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk5_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk6_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk7_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk8_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk9_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk10_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk11_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk12_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk13_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk14_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk15_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk16_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk17_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk18_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk19_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk20_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk21_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk22_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk23_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk24_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk25_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk26_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk27_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk28_AtkDistType: u8,
	#[deku(bits = 4)]
	pub atk29_AtkDistType: u8,
	#[deku(count = "13")]
	pub pad0: Vec<u8>,
}
