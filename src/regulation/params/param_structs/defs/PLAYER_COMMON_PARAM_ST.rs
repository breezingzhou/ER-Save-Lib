use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct PLAYER_COMMON_PARAM_ST {
	pub playerFootEffect_bySFX: i32,
	pub snipeModeDrawAlpha_FadeTime: f32,
	pub toughnessRecoverCorrection: f32,
	pub baseMagicSlotSize: u8,
	pub baseAccSlotNum: u8,
	pub reserved02: [u8;2],
	pub animeID_DropItemPick: i32,
	pub resistRecoverPoint_Sleep_Player: f32,
	pub flareOverrideHomingAngle: i32,
	pub flareOverrideHomingStopRange: f32,
	pub animeID_SleepCollectorItemPick: i32,
	pub unlockEventFlagBaseId_forWepAttr: i32,
	pub systemEnchant_BigRune: i32,
	pub lowStatus_AtkPowDown: f32,
	pub lowStatus_ConsumeStaminaRate: f32,
	pub lowStatus_AtkGuardBreak: i16,
	pub guardStatusCorrect_MaxStatusVal: i16,
	pub unlockEventFlagStepNum_forWepAttr: i16,
	pub retributionMagic_damageCountNum: i16,
	pub retributionMagic_damageCountRemainTime: i16,
	pub retributionMagic_burstDmypolyId: i16,
	pub retributionMagic_burstMagicParamId: i32,
	pub chrAimCam_rideOffsetHeight: f32,
	pub reserved23: [u8;4],
	pub arrowCaseWepBindDmypolyId: i32,
	pub boltPouchWepBindDmypolyId: i32,
	pub estusFlaskAllocateRate: f32,
	pub reserved38: [u8;2],
	pub kickAcceptanceDeg: u8,
	pub npcPlayerAnalogWeightRate_Light: u8,
	pub npcPlayerAnalogWeightRate_Normal: u8,
	pub npcPlayerAnalogWeightRate_Heavy: u8,
	pub npcPlayerAnalogWeightRate_WeightOver: u8,
	pub npcPlayerAnalogWeightRate_SuperLight: u8,
	pub reserved45: [u8;4],
	pub clearCountCorrectBaseSpEffectId: i32,
	pub arrowBoltModelIdOffset: i32,
	pub arrowBoltRemainingNumModelMaskThreshold1: i8,
	pub arrowBoltRemainingNumModelMaskThreshold2: i8,
	pub reserved27: [u8;2],
	pub resistRecoverPoint_Poision_Player: f32,
	pub resistRecoverPoint_Desease_Player: f32,
	pub resistRecoverPoint_Blood_Player: f32,
	pub resistRecoverPoint_Curse_Player: f32,
	pub resistRecoverPoint_Freeze_Player: f32,
	pub resistRecoverPoint_Poision_Enemy: f32,
	pub resistRecoverPoint_Desease_Enemy: f32,
	pub resistRecoverPoint_Blood_Enemy: f32,
	pub resistRecoverPoint_Curse_Enemy: f32,
	pub resistRecoverPoint_Freeze_Enemy: f32,
	pub requestTimeLeftBothHand: f32,
	pub resistRecoverPoint_Madness_Player: f32,
	pub animeID_MaterialItemPick: i32,
	pub hpEstusFlaskAllocateRateForYellowMonk: f32,
	pub hpEstusFlaskAllocateOffsetForYellowMonk: i32,
	pub mpEstusFlaskAllocateRateForYellowMonk: f32,
	pub mpEstusFlaskAllocateOffsetForYellowMonk: i32,
	pub resistRecoverPoint_Sleep_Enemy: f32,
	pub resistRecoverPoint_Madness_Enemy: f32,
	pub resistCurseItemId: i32,
	pub resistCurseItemMaxNum: u8,
	pub reserved_123: [u8;3],
	pub resistCurseItemSpEffectBaseId: i32,
	pub resistCurseItemLotParamId_map: i32,
	#[deku(skip, cond = "version >= 11210015", count = "52")]
	pub reserved41_old: Vec<u8>,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xcc: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xd0: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xd4: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xd8: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xdc: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xe0: i32,
	#[deku(skip, cond = "version < 11210015", count = "28")]
	pub reserved41: Vec<u8>,
}
