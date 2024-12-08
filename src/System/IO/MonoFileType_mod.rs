#[cfg(feature = "System+IO+MonoFileType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonoFileType {
    Char = 2i32,
    Disk = 1i32,
    Pipe = 3i32,
    Remote = 32768i32,
    Unknown = 0i32,
}
#[cfg(feature = "System+IO+MonoFileType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::MonoFileType => "System.IO"
    ."MonoFileType"
);
