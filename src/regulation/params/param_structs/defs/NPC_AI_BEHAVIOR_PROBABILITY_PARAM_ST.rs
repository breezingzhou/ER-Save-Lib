use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST {
	pub param000: i16,
	pub param001: i16,
	pub param002: i16,
	pub param003: i16,
	pub param004: i16,
	pub param005: i16,
	pub param006: i16,
	pub param007: i16,
	pub param008: i16,
	pub param009: i16,
	pub param010: i16,
	pub param011: i16,
	pub param012: i16,
	pub param013: i16,
	pub param014: i16,
	pub param015: i16,
	pub param016: i16,
	pub param017: i16,
	pub param018: i16,
	pub param019: i16,
	pub param020: i16,
	pub param021: i16,
	pub param022: i16,
	pub param023: i16,
	pub param024: i16,
	pub param025: i16,
	pub param026: i16,
	pub param027: i16,
	pub param028: i16,
	pub param029: i16,
	pub param030: i16,
	pub param031: i16,
	pub param032: i16,
	pub param033: i16,
	pub param034: i16,
	pub param035: i16,
	pub param036: i16,
	pub param037: i16,
	pub param038: i16,
	pub param039: i16,
	pub param040: i16,
	pub param041: i16,
	pub param042: i16,
	pub param043: i16,
	pub param044: i16,
	pub param045: i16,
	pub param046: i16,
	pub param047: i16,
	pub param048: i16,
	pub param049: i16,
	pub param050: i16,
	pub param051: i16,
	pub param052: i16,
	pub param053: i16,
	pub param054: i16,
	pub param055: i16,
	pub param056: i16,
	pub param057: i16,
	pub param058: i16,
	pub param059: i16,
	pub param060: i16,
	pub param061: i16,
	pub param062: i16,
	pub param063: i16,
	pub param064: i16,
	pub param065: i16,
	pub param066: i16,
	pub param067: i16,
	pub param068: i16,
	pub param069: i16,
	pub param070: i16,
	pub param071: i16,
	pub param072: i16,
	pub param073: i16,
	pub param074: i16,
	pub param075: i16,
	pub param076: i16,
	pub param077: i16,
	pub param078: i16,
	pub param079: i16,
	pub param080: i16,
	pub param081: i16,
	pub param082: i16,
	pub param083: i16,
	pub param084: i16,
	pub param085: i16,
	pub param086: i16,
	pub param087: i16,
	pub param088: i16,
	pub param089: i16,
	pub param090: i16,
	pub param091: i16,
	pub param092: i16,
	pub param093: i16,
	pub param094: i16,
	pub param095: i16,
	pub param096: i16,
	pub param097: i16,
	pub param098: i16,
	pub param099: i16,
	pub param100: i16,
	pub param101: i16,
	pub param102: i16,
	pub param103: i16,
	pub param104: i16,
	pub param105: i16,
	pub param106: i16,
	pub param107: i16,
	pub param108: i16,
	pub param109: i16,
	pub param110: i16,
	pub param111: i16,
	pub param112: i16,
	pub param113: i16,
	pub param114: i16,
	pub param115: i16,
	pub param116: i16,
	pub param117: i16,
	pub param118: i16,
	pub param119: i16,
	pub param120: i16,
	pub param121: i16,
	pub param122: i16,
	pub param123: i16,
	pub param124: i16,
	pub param125: i16,
	pub param126: i16,
	pub param127: i16,
	pub param128: i16,
	pub param129: i16,
	pub param130: i16,
	pub param131: i16,
	pub param132: i16,
	pub param133: i16,
	pub param134: i16,
	pub param135: i16,
	pub param136: i16,
	pub param137: i16,
	pub param138: i16,
	pub param139: i16,
	pub param140: i16,
	pub param141: i16,
	pub param142: i16,
	pub param143: i16,
	pub param144: i16,
	pub param145: i16,
	pub param146: i16,
	pub param147: i16,
	pub param148: i16,
	pub param149: i16,
	pub param150: i16,
	pub param151: i16,
	pub param152: i16,
	pub param153: i16,
	pub param154: i16,
	pub param155: i16,
	pub param156: i16,
	pub param157: i16,
	pub param158: i16,
	pub param159: i16,
	pub param160: i16,
	pub param161: i16,
	pub param162: i16,
	pub param163: i16,
	pub param164: i16,
	pub param165: i16,
	pub param166: i16,
	pub param167: i16,
	pub param168: i16,
	pub param169: i16,
	pub param170: i16,
	pub param171: i16,
	pub param172: i16,
	pub param173: i16,
	pub param174: i16,
	pub param175: i16,
	pub param176: i16,
	pub param177: i16,
	pub param178: i16,
	pub param179: i16,
	pub param180: i16,
	pub param181: i16,
	pub param182: i16,
	pub param183: i16,
	pub param184: i16,
	pub param185: i16,
	pub param186: i16,
	pub param187: i16,
	pub param188: i16,
	pub param189: i16,
	pub param190: i16,
	pub param191: i16,
	pub param192: i16,
	pub param193: i16,
	pub param194: i16,
	pub param195: i16,
	pub param196: i16,
	pub param197: i16,
	pub param198: i16,
	pub param199: i16,
}
