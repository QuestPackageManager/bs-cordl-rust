#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphAnchorPoint")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GlyphAnchorPoint {
    pub m_XCoordinate: f32,
    pub m_YCoordinate: f32,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphAnchorPoint")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint =>
    "UnityEngine.TextCore.LowLevel"."GlyphAnchorPoint"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphAnchorPoint")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphAnchorPoint")]
impl crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint {
    pub fn get_xCoordinate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xCoordinate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yCoordinate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yCoordinate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
