#[cfg(feature = "System+Runtime+InteropServices+DllImportSearchPath")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DllImportSearchPath {
    ApplicationDirectory = 512i32,
    AssemblyDirectory = 2i32,
    LegacyBehavior = 0i32,
    SafeDirectories = 4096i32,
    System32 = 2048i32,
    UseDllDirectoryForDependencies = 256i32,
    UserDirectories = 1024i32,
}
#[cfg(feature = "System+Runtime+InteropServices+DllImportSearchPath")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::DllImportSearchPath =>
    "System.Runtime.InteropServices"."DllImportSearchPath"
);
