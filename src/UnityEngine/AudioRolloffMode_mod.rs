#[cfg(feature = "UnityEngine+AudioRolloffMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioRolloffMode {
    #[default]
    Custom = 2i32,
    Linear = 1i32,
    Logarithmic = 0i32,
}
#[cfg(feature = "UnityEngine+AudioRolloffMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioRolloffMode => "UnityEngine"
    ."AudioRolloffMode"
);
