use super::ByteUnit;

pub type BYTES = u64;

pub const MAX_SIZE_LEN: usize = 7;

pub const B: BYTES = 1;
pub const KB: BYTES = B * 1000;
pub const MB: BYTES = KB * 1000;
pub const GB: BYTES = MB * 1000;
pub const TB: BYTES = GB * 1000;
pub const PB: BYTES = TB * 1000;

pub const KIB: BYTES = B * 1_024;
pub const MIB: BYTES = KIB * 1024;
pub const GIB: BYTES = MIB * 1024;
pub const TIB: BYTES = GIB * 1024;
pub const PIB: BYTES = TIB * 1024;

pub const UNITS_SI: [ByteUnit<'static>; 6] = [
    ByteUnit::from_tuple((B, "b")),
    ByteUnit::from_tuple((KB, "k")),
    ByteUnit::from_tuple((MB, "m")),
    ByteUnit::from_tuple((GB, "g")),
    ByteUnit::from_tuple((TB, "t")),
    ByteUnit::from_tuple((PB, "p")),
];
pub const UNITS: [ByteUnit<'static>; 6] = [
    ByteUnit::from_tuple((B, "B")),
    ByteUnit::from_tuple((KIB, "KiB")),
    ByteUnit::from_tuple((MIB, "MiB")),
    ByteUnit::from_tuple((GIB, "GiB")),
    ByteUnit::from_tuple((TIB, "TiB")),
    ByteUnit::from_tuple((PIB, "PiB")),
];
