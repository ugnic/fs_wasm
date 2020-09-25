pub(crate) fn swap_endian_from_u32(byte: u32) -> u32 { (byte << 24) | ((byte & 0x0000ff00) << 8) | ((byte & 0x00ff0000) >> 8) | (byte >> 24) }

pub(crate) fn swap_endian_from_u16(byte: u16) -> u16 { (byte << 8) | (byte >> 8) }