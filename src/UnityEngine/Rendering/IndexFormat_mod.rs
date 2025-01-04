#[cfg(feature = "UnityEngine+Rendering+IndexFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IndexFormat {
    #[default]
    UInt16 = 0i32,
    UInt32 = 1i32,
}
#[cfg(feature = "UnityEngine+Rendering+IndexFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::IndexFormat =>
    "UnityEngine.Rendering"."IndexFormat"
);
