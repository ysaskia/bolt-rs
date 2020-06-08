// HIGH AND LOW NIBBLE
pub const HIGH_NIBBLE:u8                 = 0xF0;
pub const LOW_NIBBLE:u8                  = 0x0F;
// PACKSTREAM MARKERS
pub const TINY_INT_PF:u8                 = 0x00;
pub const TINY_INT_PL:u8                 = 0x7F;
pub const TINY_STRING:u8                 = 0x80;
pub const TINY_LIST:u8                   = 0x90;
pub const TINY_MAP:u8                    = 0xA0;
pub const TINY_STRUCT:u8                 = 0xB0;
pub const NULL:u8                        = 0xC0;
pub const FLOAT_64:u8                    = 0xC1;
pub const FALSE:u8                       = 0xC2;
pub const TRUE:u8                        = 0xC3;
pub const INT_8:u8                       = 0xC8;
pub const INT_16:u8                      = 0xC9;
pub const INT_32:u8                      = 0xCA;
pub const INT_64:u8                      = 0xCB;
pub const BYTES_8:u8                     = 0xCC;
pub const BYTES_16:u8                    = 0xCD;
pub const BYTES_32:u8                    = 0xCE;
pub const STRING_8:u8                    = 0xD0;
pub const STRING_16:u8                   = 0xD1;
pub const STRING_32:u8                   = 0xD2;
pub const LIST_8:u8                      = 0xD4;
pub const LIST_16:u8                     = 0xD5;
pub const LIST_32:u8                     = 0xD6;
pub const LIST_STREAM:u8                 = 0xD7;
pub const MAP_8:u8                       = 0xD8;
pub const MAP_16:u8                      = 0xD9;
pub const MAP_32:u8                      = 0xDA;
pub const MAP_STREAM:u8                  = 0xDB;
pub const STRUCT_8:u8                    = 0xDC;
pub const STRUCT_16:u8                   = 0xDD;
pub const END_OF_STREAM:u8               = 0xDF;
pub const TINY_INT_NF:u8                 = 0xF0;
pub const TINY_INT_NL:u8                 = 0xFF;
// GRAPH STRUCT SIGNATURES
pub const STRUCT_NODE:u8                 = 0x4E;
pub const STRUCT_RELATIONSHIP:u8         = 0x52;
pub const STRUCT_PATH:u8                 = 0x50;
pub const STRUCT_UNBOUND_RELATIONSHIP:u8 = 0x72;
// MESSAGE STRUCT SIGNATURES
pub const MSG_INIT:u8                    = 0x01;
pub const MSG_RUN:u8                     = 0x10;
pub const MSG_DISCARD_ALL:u8             = 0x2F;
pub const MSG_PULL_ALL:u8                = 0x3F;
pub const MSG_ACK_FAILURE:u8             = 0x0E;
pub const MSG_RESET:u8                   = 0x0F;
pub const MSG_RECORD:u8                  = 0x71;
pub const MSG_SUCCESS:u8                 = 0x70;
pub const MSG_FAILURE:u8                 = 0x7F;
pub const MSG_IGNORED:u8                 = 0x7E;
// LIST / MAP SIZES CONSTANTS
pub const EMPTY_SIZE:i32   = 0;
pub const UNKNOWN_SIZE:i32 = -1;
// TINY INT MIN AND MAX
pub const TINY_MIN:i8 = -16;
pub const TINY_MAX:i8 =  15;
// BIT CONVERSION
pub const SHIFT_64: [u8; 8] = [56, 48, 40, 32, 24, 16, 8, 0];
pub const SHIFT_32: [u8; 4] = [24, 16, 8, 0];
pub const SHIFT_16: [u8; 2] = [8, 0];
