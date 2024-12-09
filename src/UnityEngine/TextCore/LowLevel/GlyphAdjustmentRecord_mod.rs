#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphAdjustmentRecord")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GlyphAdjustmentRecord {
    pub m_GlyphIndex: u32,
    pub m_GlyphValueRecord: crate::UnityEngine::TextCore::LowLevel::GlyphValueRecord,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphAdjustmentRecord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord =>
    "UnityEngine.TextCore.LowLevel"."GlyphAdjustmentRecord"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphAdjustmentRecord")]
impl crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord {
    pub fn Equals_GlyphAdjustmentRecord1(
        &mut self,
        other: crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
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
    pub fn get_glyphIndex(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_glyphIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_glyphValueRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::GlyphValueRecord,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::GlyphValueRecord = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_glyphValueRecord",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
