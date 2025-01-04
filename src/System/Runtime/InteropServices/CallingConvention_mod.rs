#[cfg(feature = "System+Runtime+InteropServices+CallingConvention")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CallingConvention {
    #[default]
    Cdecl = 2i32,
    FastCall = 5i32,
    StdCall = 3i32,
    ThisCall = 4i32,
    Winapi = 1i32,
}
#[cfg(feature = "System+Runtime+InteropServices+CallingConvention")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::CallingConvention =>
    "System.Runtime.InteropServices"."CallingConvention"
);
