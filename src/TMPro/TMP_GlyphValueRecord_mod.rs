#[cfg(feature = "TMPro+TMP_GlyphValueRecord")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TMP_GlyphValueRecord {
    pub m_XPlacement: f32,
    pub m_YPlacement: f32,
    pub m_XAdvance: f32,
    pub m_YAdvance: f32,
}
#[cfg(feature = "TMPro+TMP_GlyphValueRecord")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_GlyphValueRecord {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_GlyphValueRecord";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::TMP_GlyphValueRecord {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::TMPro::TMP_GlyphValueRecord {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::TMP_GlyphValueRecord {
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
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::TMP_GlyphValueRecord {
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
#[cfg(feature = "TMPro+TMP_GlyphValueRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_GlyphValueRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_GlyphValueRecord")]
impl crate::TMPro::TMP_GlyphValueRecord {
    pub fn _ctor_GlyphValueRecord2(
        &mut self,
        valueRecord: crate::UnityEngine::TextCore::LowLevel::GlyphValueRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (valueRecord),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GlyphValueRecord_Legacy1(
        &mut self,
        valueRecord: crate::TMPro::GlyphValueRecord_Legacy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (valueRecord),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_f32_f32_0(
        &mut self,
        xPlacement: f32,
        yPlacement: f32,
        xAdvance: f32,
        yAdvance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (xPlacement, yPlacement, xAdvance, yAdvance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xAdvance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xAdvance",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xPlacement(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xPlacement",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yAdvance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yAdvance",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yPlacement(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yPlacement",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        a: crate::TMPro::TMP_GlyphValueRecord,
        b: crate::TMPro::TMP_GlyphValueRecord,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_GlyphValueRecord> {
        let __cordl_ret: crate::TMPro::TMP_GlyphValueRecord = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xAdvance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xAdvance",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xPlacement(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xPlacement",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yAdvance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yAdvance",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yPlacement(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yPlacement",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
