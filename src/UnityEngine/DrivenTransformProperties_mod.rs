#[cfg(feature = "UnityEngine+DrivenTransformProperties")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrivenTransformProperties {
    #[default]
    All = -1i32,
    AnchorMax = 3072i32,
    AnchorMaxX = 1024i32,
    AnchorMaxY = 2048i32,
    AnchorMin = 768i32,
    AnchorMinX = 256i32,
    AnchorMinY = 512i32,
    AnchoredPosition = 6i32,
    AnchoredPosition3D = 14i32,
    AnchoredPositionX = 2i32,
    AnchoredPositionY = 4i32,
    AnchoredPositionZ = 8i32,
    Anchors = 3840i32,
    None = 0i32,
    Pivot = 49152i32,
    PivotX = 16384i32,
    PivotY = 32768i32,
    Rotation = 16i32,
    Scale = 224i32,
    ScaleX = 32i32,
    ScaleY = 64i32,
    ScaleZ = 128i32,
    SizeDelta = 12288i32,
    SizeDeltaX = 4096i32,
    SizeDeltaY = 8192i32,
}
#[cfg(feature = "UnityEngine+DrivenTransformProperties")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::DrivenTransformProperties {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "DrivenTransformProperties";
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
#[cfg(feature = "UnityEngine+DrivenTransformProperties")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::DrivenTransformProperties {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+DrivenTransformProperties")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::DrivenTransformProperties {
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
#[cfg(feature = "UnityEngine+DrivenTransformProperties")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::DrivenTransformProperties {
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
#[cfg(feature = "UnityEngine+DrivenTransformProperties")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::DrivenTransformProperties {
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
