#[cfg(feature = "UnityEngine+Bindings+CodegenOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodegenOptions {
    Auto = 0i32,
    Custom = 1i32,
    Force = 2i32,
}
#[cfg(feature = "UnityEngine+Bindings+CodegenOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Bindings::CodegenOptions =>
    "UnityEngine.Bindings"."CodegenOptions"
);
