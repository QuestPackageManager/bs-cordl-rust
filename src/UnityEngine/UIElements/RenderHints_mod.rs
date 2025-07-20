#[cfg(feature = "UnityEngine+UIElements+RenderHints")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderHints {
    #[default]
    BoneTransform = 2i32,
    ClipWithScissors = 4i32,
    DirtyAll = 992i32,
    DirtyBoneTransform = 64i32,
    DirtyClipWithScissors = 128i32,
    DirtyDynamicColor = 512i32,
    DirtyGroupTransform = 32i32,
    DirtyMaskContainer = 256i32,
    DirtyOffset = 5i32,
    DynamicColor = 16i32,
    GroupTransform = 1i32,
    MaskContainer = 8i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+RenderHints")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::RenderHints {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "RenderHints";
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
#[cfg(feature = "UnityEngine+UIElements+RenderHints")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::RenderHints {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+RenderHints")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::RenderHints {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+RenderHints")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::RenderHints {
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
#[cfg(feature = "UnityEngine+UIElements+RenderHints")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::RenderHints {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
