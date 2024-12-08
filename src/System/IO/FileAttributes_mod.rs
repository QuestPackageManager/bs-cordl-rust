#[cfg(feature = "System+IO+FileAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAttributes {
    Archive = 32i32,
    Compressed = 2048i32,
    Device = 64i32,
    Directory = 16i32,
    Encrypted = 16384i32,
    Hidden = 2i32,
    IntegrityStream = 32768i32,
    NoScrubData = 131072i32,
    Normal = 128i32,
    NotContentIndexed = 8192i32,
    Offline = 4096i32,
    ReadOnly = 1i32,
    ReparsePoint = 1024i32,
    SparseFile = 512i32,
    System = 4i32,
    Temporary = 256i32,
}
#[cfg(feature = "System+IO+FileAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::FileAttributes => "System.IO"
    ."FileAttributes"
);
