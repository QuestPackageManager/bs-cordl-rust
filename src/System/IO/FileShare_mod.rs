#[cfg(feature = "System+IO+FileShare")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileShare {
    Delete = 4i32,
    Inheritable = 16i32,
    None = 0i32,
    Read = 1i32,
    ReadWrite = 3i32,
    Write = 2i32,
}
#[cfg(feature = "System+IO+FileShare")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::FileShare => "System.IO"."FileShare"
);
