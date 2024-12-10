#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToMarkAdjustmentRecord")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MarkToMarkAdjustmentRecord {
    pub m_BaseMarkGlyphID: u32,
    pub m_BaseMarkGlyphAnchorPoint: crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint,
    pub m_CombiningMarkGlyphID: u32,
    pub m_CombiningMarkPositionAdjustment: crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToMarkAdjustmentRecord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord =>
    "UnityEngine.TextCore.LowLevel"."MarkToMarkAdjustmentRecord"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToMarkAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToMarkAdjustmentRecord")]
impl crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord {
    pub fn get_baseMarkGlyphAnchorPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_baseMarkGlyphAnchorPoint",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_baseMarkGlyphID(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_baseMarkGlyphID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_combiningMarkGlyphID(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_combiningMarkGlyphID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_combiningMarkPositionAdjustment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_combiningMarkPositionAdjustment",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
