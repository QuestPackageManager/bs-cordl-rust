#[cfg(feature = "UnityEngine+IMECompositionMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IMECompositionMode {
    #[default]
    Auto = 0i32,
    Off = 2i32,
    On = 1i32,
}
#[cfg(feature = "UnityEngine+IMECompositionMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::IMECompositionMode => "UnityEngine"
    ."IMECompositionMode"
);
