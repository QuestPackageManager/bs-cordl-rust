#[cfg(feature = "System+IO+FileOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileOptions {
    #[default]
    Asynchronous = 1073741824i32,
    DeleteOnClose = 67108864i32,
    Encrypted = 16384i32,
    None = 0i32,
    RandomAccess = 268435456i32,
    SequentialScan = 134217728i32,
    WriteThrough = -2147483648i32,
}
#[cfg(feature = "System+IO+FileOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::FileOptions => "System.IO"
    ."FileOptions"
);
