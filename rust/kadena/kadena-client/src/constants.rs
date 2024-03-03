/// The number of decimal places to shift to convert KDA to its smallest unit, picoKDA
pub const KDA_DECIMAL_PLACES: u32 = 12;

/// The scaling factor to convert from KDA to picoKDA
pub const KDA_SCALING_FACTOR: u64 = 10u64.pow(KDA_DECIMAL_PLACES);
