#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToBaseAdjustmentRecord")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MarkToBaseAdjustmentRecord {
    pub m_BaseGlyphID: u32,
    pub m_BaseGlyphAnchorPoint: crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint,
    pub m_MarkGlyphID: u32,
    pub m_MarkPositionAdjustment: crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToBaseAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.LowLevel";
    const CLASS_NAME: &'static str = "MarkToBaseAdjustmentRecord";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord {
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
