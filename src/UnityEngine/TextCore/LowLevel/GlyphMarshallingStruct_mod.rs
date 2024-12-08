#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphMarshallingStruct")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GlyphMarshallingStruct {
    pub index: u32,
    pub metrics: crate::UnityEngine::TextCore::GlyphMetrics,
    pub glyphRect: crate::UnityEngine::TextCore::GlyphRect,
    pub scale: f32,
    pub atlasIndex: i32,
    pub classDefinitionType: crate::UnityEngine::TextCore::GlyphClassDefinitionType,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphMarshallingStruct")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct =>
    "UnityEngine.TextCore.LowLevel"."GlyphMarshallingStruct"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphMarshallingStruct")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphMarshallingStruct")]
impl crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct {}
