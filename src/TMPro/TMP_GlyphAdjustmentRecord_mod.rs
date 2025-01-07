#[cfg(feature = "TMPro+TMP_GlyphAdjustmentRecord")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TMP_GlyphAdjustmentRecord {
    pub m_GlyphIndex: u32,
    pub m_GlyphValueRecord: crate::TMPro::TMP_GlyphValueRecord,
}
#[cfg(feature = "TMPro+TMP_GlyphAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_GlyphAdjustmentRecord {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_GlyphAdjustmentRecord";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::TMP_GlyphAdjustmentRecord {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::TMPro::TMP_GlyphAdjustmentRecord {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::TMP_GlyphAdjustmentRecord {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::TMP_GlyphAdjustmentRecord {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
    pub fn _ctor_GlyphAdjustmentRecord1(
        &mut self,
        adjustmentRecord: crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (adjustmentRecord),
        )?;
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
    }
    pub fn get_glyphIndex(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_glyphIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_glyphValueRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_GlyphValueRecord> {
        let __cordl_ret: crate::TMPro::TMP_GlyphValueRecord = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_glyphValueRecord",
            (),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
