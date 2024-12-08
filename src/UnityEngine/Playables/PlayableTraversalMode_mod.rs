#[cfg(feature = "UnityEngine+Playables+PlayableTraversalMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayableTraversalMode {
    Mix = 0i32,
    Passthrough = 1i32,
}
#[cfg(feature = "UnityEngine+Playables+PlayableTraversalMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayableTraversalMode =>
    "UnityEngine.Playables"."PlayableTraversalMode"
);
