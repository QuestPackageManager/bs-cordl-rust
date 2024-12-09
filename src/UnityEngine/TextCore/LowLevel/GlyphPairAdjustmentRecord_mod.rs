#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GlyphPairAdjustmentRecord {
    pub m_FirstAdjustmentRecord: crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
    pub m_SecondAdjustmentRecord: crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
    pub m_FeatureLookupFlags: crate::UnityEngine::TextCore::LowLevel::FontFeatureLookupFlags,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord =>
    "UnityEngine.TextCore.LowLevel"."GlyphPairAdjustmentRecord"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
impl crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord {
    pub fn Equals_GlyphPairAdjustmentRecord1(
        &mut self,
        other: crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_featureLookupFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontFeatureLookupFlags,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontFeatureLookupFlags = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_featureLookupFlags",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_firstAdjustmentRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_firstAdjustmentRecord",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_secondAdjustmentRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_secondAdjustmentRecord",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
