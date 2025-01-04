#[cfg(feature = "UnityEngine+AudioDataLoadState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioDataLoadState {
    #[default]
    Failed = 3i32,
    Loaded = 2i32,
    Loading = 1i32,
    Unloaded = 0i32,
}
#[cfg(feature = "UnityEngine+AudioDataLoadState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioDataLoadState => "UnityEngine"
    ."AudioDataLoadState"
);
