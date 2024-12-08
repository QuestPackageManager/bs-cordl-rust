#[cfg(feature = "System+IO+Compression+CompressionLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    Fastest = 1i32,
    NoCompression = 2i32,
    Optimal = 0i32,
}
#[cfg(feature = "System+IO+Compression+CompressionLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Compression::CompressionLevel =>
    "System.IO.Compression"."CompressionLevel"
);
