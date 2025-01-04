#[cfg(feature = "UnityEngine+Audio+AudioMixerUpdateMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioMixerUpdateMode {
    #[default]
    Normal = 0i32,
    UnscaledTime = 1i32,
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerUpdateMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Audio::AudioMixerUpdateMode =>
    "UnityEngine.Audio"."AudioMixerUpdateMode"
);
