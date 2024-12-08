#[cfg(feature = "System+Runtime+InteropServices+Architecture")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    Arm = 2i32,
    Arm64 = 3i32,
    X64 = 1i32,
    X86 = 0i32,
}
#[cfg(feature = "System+Runtime+InteropServices+Architecture")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::Architecture
    => "System.Runtime.InteropServices"."Architecture"
);
