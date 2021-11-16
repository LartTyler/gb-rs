pub mod bytes;

pub enum DeviceMode {
    Color,
    Classic,
    Any,
}

pub const CPU_CLOCK_FREQ_NORMAL: u64 = 4_194_304;
pub const CPU_CLOCK_FREQ_DOUBLE: u64 = CPU_CLOCK_FREQ_NORMAL * 2;
