#[cfg(feature = "UnityEngine+Playables+PlayState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayState {
    Delayed = 2i32,
    Paused = 0i32,
    Playing = 1i32,
}
#[cfg(feature = "UnityEngine+Playables+PlayState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayState =>
    "UnityEngine.Playables"."PlayState"
);
