#[cfg(feature = "TMPro+GlyphPairKey")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GlyphPairKey {
    pub firstGlyphIndex: u32,
    pub secondGlyphIndex: u32,
    pub key: u32,
}
#[cfg(feature = "TMPro+GlyphPairKey")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::GlyphPairKey => "TMPro"."GlyphPairKey"
);
#[cfg(feature = "TMPro+GlyphPairKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::GlyphPairKey {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+GlyphPairKey")]
impl crate::TMPro::GlyphPairKey {
    pub fn _ctor_TMP_GlyphPairAdjustmentRecord1(
        &mut self,
        record: *mut crate::TMPro::TMP_GlyphPairAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (record),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u32_u32_0(
        &mut self,
        firstGlyphIndex: u32,
        secondGlyphIndex: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (firstGlyphIndex, secondGlyphIndex),
        )?;
        Ok(__cordl_ret)
    }
}
