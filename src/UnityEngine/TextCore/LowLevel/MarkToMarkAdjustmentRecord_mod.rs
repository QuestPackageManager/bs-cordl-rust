#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToMarkAdjustmentRecord")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MarkToMarkAdjustmentRecord {
    pub m_BaseMarkGlyphID: u32,
    pub m_BaseMarkGlyphAnchorPoint: crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint,
    pub m_CombiningMarkGlyphID: u32,
    pub m_CombiningMarkPositionAdjustment: crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToMarkAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.LowLevel";
    const CLASS_NAME: &'static str = "MarkToMarkAdjustmentRecord";
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
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToMarkAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToMarkAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord {
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
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToMarkAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord {
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
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkToMarkAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint,
                0usize,
            >("get_baseMarkGlyphAnchorPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_baseMarkGlyphAnchorPoint", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::GlyphAnchorPoint = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_baseMarkGlyphID(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u32, 0usize>("get_baseMarkGlyphID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_baseMarkGlyphID", 0usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_combiningMarkGlyphID(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u32, 0usize>("get_combiningMarkGlyphID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_combiningMarkGlyphID", 0usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_combiningMarkPositionAdjustment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment,
                0usize,
            >("get_combiningMarkPositionAdjustment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_combiningMarkPositionAdjustment", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
