#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToBaseAdjustmentRecord")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MarkToBaseAdjustmentRecord {
    pub m_BaseGlyphID: u32,
    pub m_BaseGlyphAnchorPoint: crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint,
    pub m_MarkGlyphID: u32,
    pub m_MarkPositionAdjustment: crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToBaseAdjustmentRecord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord =>
    "UnityEngine.TextCore.LowLevel"."MarkToBaseAdjustmentRecord"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToBaseAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToBaseAdjustmentRecord")]
impl crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord {
    pub fn get_baseGlyphAnchorPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_baseGlyphAnchorPoint",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_baseGlyphID(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_baseGlyphID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_markGlyphID(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_markGlyphID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_markPositionAdjustment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_markPositionAdjustment",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
