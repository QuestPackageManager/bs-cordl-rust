#[cfg(feature = "UnityEngine+Tilemaps+TileAnimationData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TileAnimationData {
    pub m_AnimatedSprites: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        >,
    >,
    pub m_AnimationSpeed: f32,
    pub m_AnimationStartTime: f32,
    pub m_Flags: crate::UnityEngine::Tilemaps::TileAnimationFlags,
}
#[cfg(feature = "UnityEngine+Tilemaps+TileAnimationData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Tilemaps::TileAnimationData =>
    "UnityEngine.Tilemaps"."TileAnimationData"
);
#[cfg(feature = "UnityEngine+Tilemaps+TileAnimationData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Tilemaps::TileAnimationData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TileAnimationData")]
impl crate::UnityEngine::Tilemaps::TileAnimationData {}
