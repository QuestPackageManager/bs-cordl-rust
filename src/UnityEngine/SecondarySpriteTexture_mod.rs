#[cfg(feature = "UnityEngine+SecondarySpriteTexture")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SecondarySpriteTexture {
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub texture: *mut crate::UnityEngine::Texture2D,
}
#[cfg(feature = "UnityEngine+SecondarySpriteTexture")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SecondarySpriteTexture =>
    "UnityEngine"."SecondarySpriteTexture"
);
#[cfg(feature = "UnityEngine+SecondarySpriteTexture")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::SecondarySpriteTexture {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SecondarySpriteTexture")]
impl crate::UnityEngine::SecondarySpriteTexture {}
