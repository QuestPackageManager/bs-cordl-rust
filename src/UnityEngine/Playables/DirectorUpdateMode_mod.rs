#[cfg(feature = "UnityEngine+Playables+DirectorUpdateMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectorUpdateMode {
    DSPClock = 0i32,
    GameTime = 1i32,
    Manual = 3i32,
    UnscaledGameTime = 2i32,
}
#[cfg(feature = "UnityEngine+Playables+DirectorUpdateMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::DirectorUpdateMode =>
    "UnityEngine.Playables"."DirectorUpdateMode"
);
