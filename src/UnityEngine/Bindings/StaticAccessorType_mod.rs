#[cfg(feature = "UnityEngine+Bindings+StaticAccessorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StaticAccessorType {
    #[default]
    Arrow = 1i32,
    ArrowWithDefaultReturnIfNull = 3i32,
    Dot = 0i32,
    DoubleColon = 2i32,
}
#[cfg(feature = "UnityEngine+Bindings+StaticAccessorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Bindings::StaticAccessorType =>
    "UnityEngine.Bindings"."StaticAccessorType"
);
