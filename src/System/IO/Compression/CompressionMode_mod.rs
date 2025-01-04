#[cfg(feature = "System+IO+Compression+CompressionMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionMode {
    #[default]
    Compress = 1i32,
    Decompress = 0i32,
}
#[cfg(feature = "System+IO+Compression+CompressionMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Compression::CompressionMode =>
    "System.IO.Compression"."CompressionMode"
);
