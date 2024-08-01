use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct FOOT_SFX_PARAM_ST {
	pub sfxId_00: i32,
	pub sfxId_01: i32,
	pub sfxId_02: i32,
	pub sfxId_03: i32,
	pub sfxId_04: i32,
	pub sfxId_05: i32,
	pub sfxId_06: i32,
	pub sfxId_07: i32,
	pub sfxId_08: i32,
	pub sfxId_09: i32,
	pub sfxId_10: i32,
	pub sfxId_11: i32,
	pub sfxId_12: i32,
	pub sfxId_13: i32,
	pub sfxId_14: i32,
	pub sfxId_15: i32,
	pub sfxId_16: i32,
	pub sfxId_17: i32,
	pub sfxId_18: i32,
	pub sfxId_19: i32,
	pub sfxId_20: i32,
	pub sfxId_21: i32,
	pub sfxId_22: i32,
	pub sfxId_23: i32,
	pub sfxId_24: i32,
	pub sfxId_25: i32,
	pub sfxId_26: i32,
	pub sfxId_27: i32,
	pub sfxId_28: i32,
	pub sfxId_29: i32,
	pub sfxId_30: i32,
	pub sfxId_31: i32,
	pub sfxId_32: i32,
	pub sfxId_33: i32,
	pub sfxId_34: i32,
	pub sfxId_35: i32,
	pub sfxId_36: i32,
	pub sfxId_37: i32,
	pub sfxId_38: i32,
	pub sfxId_39: i32,
	pub sfxId_40: i32,
	pub sfxId_41: i32,
	pub sfxId_42: i32,
	pub sfxId_43: i32,
	pub sfxId_44: i32,
	pub sfxId_45: i32,
	pub sfxId_46: i32,
	pub sfxId_47: i32,
	pub sfxId_48: i32,
	pub sfxId_49: i32,
	pub sfxId_50: i32,
	pub sfxId_51: i32,
	pub sfxId_52: i32,
	pub sfxId_53: i32,
	pub sfxId_54: i32,
	pub sfxId_55: i32,
	pub sfxId_56: i32,
	pub sfxId_57: i32,
	pub sfxId_58: i32,
	pub sfxId_59: i32,
	pub sfxId_60: i32,
	pub sfxId_61: i32,
	pub sfxId_62: i32,
	pub sfxId_63: i32,
	pub sfxId_64: i32,
	pub sfxId_65: i32,
	pub sfxId_66: i32,
	pub sfxId_67: i32,
	pub sfxId_68: i32,
	pub sfxId_69: i32,
	pub sfxId_70: i32,
	pub sfxId_71: i32,
	pub sfxId_72: i32,
	pub sfxId_73: i32,
	pub sfxId_74: i32,
	pub sfxId_75: i32,
	pub sfxId_76: i32,
	pub sfxId_77: i32,
	pub sfxId_78: i32,
	pub sfxId_79: i32,
	pub sfxId_80: i32,
	pub sfxId_81: i32,
	pub sfxId_82: i32,
	pub sfxId_83: i32,
	pub sfxId_84: i32,
	pub sfxId_85: i32,
	pub sfxId_86: i32,
	pub sfxId_87: i32,
	pub sfxId_88: i32,
	pub sfxId_89: i32,
	pub sfxId_90: i32,
	pub sfxId_91: i32,
	pub sfxId_92: i32,
	pub sfxId_93: i32,
	pub sfxId_94: i32,
	pub sfxId_95: i32,
	pub sfxId_96: i32,
	pub sfxId_97: i32,
	pub sfxId_98: i32,
	pub sfxId_99: i32,
	pub sfxId_100: i32,
	pub sfxId_101: i32,
	pub sfxId_102: i32,
	pub sfxId_103: i32,
	pub sfxId_104: i32,
	pub sfxId_105: i32,
	pub sfxId_106: i32,
	pub sfxId_107: i32,
	pub sfxId_108: i32,
	pub sfxId_109: i32,
	pub sfxId_110: i32,
	pub sfxId_111: i32,
	pub sfxId_112: i32,
	pub sfxId_113: i32,
	pub sfxId_114: i32,
	pub sfxId_115: i32,
	pub sfxId_116: i32,
	pub sfxId_117: i32,
	pub sfxId_118: i32,
	pub sfxId_119: i32,
	pub sfxId_120: i32,
	pub sfxId_121: i32,
	pub sfxId_122: i32,
	pub sfxId_123: i32,
	pub sfxId_124: i32,
	pub sfxId_125: i32,
	pub sfxId_126: i32,
	pub sfxId_127: i32,
	pub sfxId_128: i32,
	pub sfxId_129: i32,
	pub sfxId_130: i32,
	pub sfxId_131: i32,
	pub sfxId_132: i32,
	pub sfxId_133: i32,
	pub sfxId_134: i32,
	pub sfxId_135: i32,
	pub sfxId_136: i32,
	pub sfxId_137: i32,
	pub sfxId_138: i32,
	pub sfxId_139: i32,
	pub sfxId_140: i32,
	pub sfxId_141: i32,
	pub sfxId_142: i32,
	pub sfxId_143: i32,
	pub sfxId_144: i32,
	pub sfxId_145: i32,
	pub sfxId_146: i32,
	pub sfxId_147: i32,
	pub sfxId_148: i32,
	pub sfxId_149: i32,
	pub sfxId_150: i32,
	pub sfxId_151: i32,
	pub sfxId_152: i32,
	pub sfxId_153: i32,
	pub sfxId_154: i32,
	pub sfxId_155: i32,
	pub sfxId_156: i32,
	pub sfxId_157: i32,
	pub sfxId_158: i32,
	pub sfxId_159: i32,
	pub sfxId_160: i32,
	pub sfxId_161: i32,
	pub sfxId_162: i32,
	pub sfxId_163: i32,
	pub sfxId_164: i32,
	pub sfxId_165: i32,
	pub sfxId_166: i32,
	pub sfxId_167: i32,
	pub sfxId_168: i32,
	pub sfxId_169: i32,
	pub sfxId_170: i32,
	pub sfxId_171: i32,
	pub sfxId_172: i32,
	pub sfxId_173: i32,
	pub sfxId_174: i32,
	pub sfxId_175: i32,
	pub sfxId_176: i32,
	pub sfxId_177: i32,
	pub sfxId_178: i32,
	pub sfxId_179: i32,
	pub sfxId_180: i32,
	pub sfxId_181: i32,
	pub sfxId_182: i32,
	pub sfxId_183: i32,
	pub sfxId_184: i32,
	pub sfxId_185: i32,
	pub sfxId_186: i32,
	pub sfxId_187: i32,
	pub sfxId_188: i32,
	pub sfxId_189: i32,
	pub sfxId_190: i32,
	pub sfxId_191: i32,
	pub sfxId_192: i32,
	pub sfxId_193: i32,
	pub sfxId_194: i32,
	pub sfxId_195: i32,
	pub sfxId_196: i32,
	pub sfxId_197: i32,
	pub sfxId_198: i32,
	pub sfxId_199: i32,
}