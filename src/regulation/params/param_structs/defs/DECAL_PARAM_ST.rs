use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct DECAL_PARAM_ST {
	pub textureId: i32,
	pub dmypolyId: i32,
	pub pitchAngle: f32,
	pub yawAngle: f32,
	pub nearDistance: f32,
	pub farDistance: f32,
	pub nearSize: f32,
	pub farSize: f32,
	pub maskSpeffectId: i32,
	#[deku(bits = 4)]
	pub pad_10: i32,
	#[deku(bits = 1)]
	pub replaceTextureId_byMaterial: i32,
	#[deku(bits = 2)]
	pub dmypolyCategory: i32,
	#[deku(bits = 4)]
	pub pad_05: i32,
	#[deku(bits = 1)]
	pub useDeferredDecal: i32,
	#[deku(bits = 1)]
	pub usePaintDecal: i32,
	#[deku(bits = 1)]
	pub bloodTypeEnable: i32,
	#[deku(bits = 1)]
	pub bUseNormal: i32,
	#[deku(bits = 1)]
	pub pad_08: i32,
	#[deku(bits = 1)]
	pub pad_09: i32,
	#[deku(bits = 1)]
	pub usePom: i32,
	#[deku(bits = 1)]
	pub useEmissive: i32,
	#[deku(bits = 1)]
	pub putVertical: i32,
	pub randomSizeMin: i16,
	pub randomSizeMax: i16,
	pub randomRollMin: f32,
	pub randomRollMax: f32,
	pub randomPitchMin: f32,
	pub randomPitchMax: f32,
	pub randomYawMin: f32,
	pub randomYawMax: f32,
	pub pomHightScale: f32,
	pub pomSampleMin: u8,
	pub pomSampleMax: u8,
	pub blendMode: i8,
	pub appearDirType: i8,
	pub emissiveValueBegin: f32,
	pub emissiveValueEnd: f32,
	pub emissiveTime: f32,
	pub bIntpEnable: u8,
	pub pad_01: [u8;3],
	pub intpIntervalDist: f32,
	pub beginIntpTextureId: i32,
	pub endIntpTextureId: i32,
	pub appearSfxId: i32,
	pub appearSfxOffsetPos: f32,
	pub maskTextureId: i32,
	pub diffuseTextureId: i32,
	pub reflecTextureId: i32,
	pub maskScale: f32,
	pub normalTextureId: i32,
	pub heightTextureId: i32,
	pub emissiveTextureId: i32,
	pub diffuseColorR: u8,
	pub diffuseColorG: u8,
	pub diffuseColorB: u8,
	pub pad_03: [u8;1],
	pub reflecColorR: u8,
	pub reflecColorG: u8,
	pub reflecColorB: u8,
	pub bLifeEnable: u8,
	pub siniScale: f32,
	pub lifeTimeSec: f32,
	pub fadeOutTimeSec: f32,
	pub priority: i16,
	pub bDistThinOutEnable: u8,
	pub bAlignedTexRandomVariationEnable: u8,
	pub distThinOutCheckDist: f32,
	pub distThinOutCheckAngleDeg: f32,
	pub distThinOutMaxNum: u8,
	pub distThinOutCheckNum: u8,
	pub delayAppearFrame: i16,
	#[deku(bits = 4)]
	pub randVaria_Diffuse: i32,
	#[deku(bits = 4)]
	pub randVaria_Mask: i32,
	#[deku(bits = 4)]
	pub randVaria_Reflec: i32,
	#[deku(bits = 4)]
	pub pad_12: i32,
	#[deku(bits = 4)]
	pub randVaria_Normal: i32,
	#[deku(bits = 4)]
	pub randVaria_Height: i32,
	#[deku(bits = 4)]
	pub randVaria_Emissive: i32,
	#[deku(bits = 4)]
	pub pad_11: i32,
	pub fadeInTimeSec: f32,
	pub thinOutOverlapMultiRadius: f32,
	pub thinOutNeighborAddRadius: f32,
	pub thinOutOverlapLimitNum: i32,
	pub thinOutNeighborLimitNum: i32,
	pub thinOutMode: i8,
	pub emissiveColorR: u8,
	pub emissiveColorG: u8,
	pub emissiveColorB: u8,
	pub maxDecalSfxCreatableSlopeAngleDeg: f32,
	#[deku(count = "40")]
	pub pad_02: Vec<u8>,
}
