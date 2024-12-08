#[cfg(feature = "System+Reflection+ProcessorArchitecture")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorArchitecture {
    Amd64 = 4i32,
    Arm = 5i32,
    IA64 = 3i32,
    MSIL = 1i32,
    None = 0i32,
    X86 = 2i32,
}
#[cfg(feature = "System+Reflection+ProcessorArchitecture")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::ProcessorArchitecture =>
    "System.Reflection"."ProcessorArchitecture"
);
