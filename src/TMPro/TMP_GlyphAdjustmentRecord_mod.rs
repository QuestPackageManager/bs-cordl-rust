#[cfg(feature = "TMPro+TMP_GlyphAdjustmentRecord")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_GlyphAdjustmentRecord {
    pub m_GlyphIndex: u32,
    pub m_GlyphValueRecord: crate::TMPro::TMP_GlyphValueRecord,
}
#[cfg(feature = "TMPro+TMP_GlyphAdjustmentRecord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_GlyphAdjustmentRecord => "TMPro"
    ."TMP_GlyphAdjustmentRecord"
);
#[cfg(feature = "TMPro+TMP_GlyphAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::TMP_GlyphAdjustmentRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_GlyphAdjustmentRecord")]
impl crate::TMPro::TMP_GlyphAdjustmentRecord {
    pub fn _ctor_u32_TMP_GlyphValueRecord0(
        &mut self,
        glyphIndex: u32,
        glyphValueRecord: crate::TMPro::TMP_GlyphValueRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (glyphIndex, glyphValueRecord),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_GlyphAdjustmentRecord1(
        &mut self,
        adjustmentRecord: crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (adjustmentRecord),
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
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_GlyphValueRecord> {
        let __cordl_ret: crate::TMPro::TMP_GlyphValueRecord = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_glyphValueRecord",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_glyphIndex(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_glyphIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_glyphValueRecord(
        &mut self,
        value: crate::TMPro::TMP_GlyphValueRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_glyphValueRecord",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
