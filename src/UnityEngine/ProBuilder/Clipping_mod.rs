#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
#[repr(C)]
#[derive(Debug)]
pub struct Clipping {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ProBuilder::Clipping {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "Clipping";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Clipping {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Clipping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
impl crate::UnityEngine::ProBuilder::Clipping {
    #[cfg(feature = "UnityEngine+ProBuilder+Clipping+OutCode")]
    pub type OutCode = crate::UnityEngine::ProBuilder::Clipping_OutCode;
    pub fn ComputeOutCode(
        rect: crate::UnityEngine::Rect,
        x: f32,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::Clipping_OutCode,
    > {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Clipping_OutCode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeOutCode", (rect, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn RectContainsLineSegment(
        rect: crate::UnityEngine::Rect,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RectContainsLineSegment", (rect, x0, y0, x1, y1))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Clipping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping+OutCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Clipping_OutCode {
    #[default]
    Bottom = 4i32,
    Inside = 0i32,
    Left = 1i32,
    Right = 2i32,
    Top = 8i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping+OutCode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::Clipping_OutCode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "OutCode";
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
#[cfg(feature = "UnityEngine+ProBuilder+Clipping+OutCode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ProBuilder::Clipping_OutCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Clipping+OutCode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ProBuilder::Clipping_OutCode {
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
#[cfg(feature = "UnityEngine+ProBuilder+Clipping+OutCode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ProBuilder::Clipping_OutCode {
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
#[cfg(feature = "UnityEngine+ProBuilder+Clipping+OutCode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ProBuilder::Clipping_OutCode {
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
