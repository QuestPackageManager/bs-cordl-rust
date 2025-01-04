#[cfg(feature = "UnityEngine+Playables+DirectorWrapMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DirectorWrapMode {
    #[default]
    Hold = 0i32,
    Loop = 1i32,
    None = 2i32,
}
#[cfg(feature = "UnityEngine+Playables+DirectorWrapMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::DirectorWrapMode =>
    "UnityEngine.Playables"."DirectorWrapMode"
);
