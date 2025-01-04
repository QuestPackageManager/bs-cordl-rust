#[cfg(feature = "UnityEngine+AudioSpeakerMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioSpeakerMode {
    #[default]
    Mode5point1 = 5i32,
    Mode7point1 = 6i32,
    Mono = 1i32,
    Prologic = 7i32,
    Quad = 3i32,
    Raw = 0i32,
    Stereo = 2i32,
    Surround = 4i32,
}
#[cfg(feature = "UnityEngine+AudioSpeakerMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioSpeakerMode => "UnityEngine"
    ."AudioSpeakerMode"
);
