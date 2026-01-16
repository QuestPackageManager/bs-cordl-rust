#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+EntryType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum EntryType {
    #[default]
    BeginStencilMask = 8u16,
    BlitAndPopRenderTexture = 18u16,
    CutRenderChain = 21u16,
    DedicatedPlaceholder = 22u16,
    DrawChildren = 7u16,
    DrawGradients = 4u16,
    DrawImmediate = 5u16,
    DrawImmediateCull = 6u16,
    DrawSolidMesh = 0u16,
    DrawTextMesh = 3u16,
    DrawTexturedMesh = 1u16,
    DrawTexturedMeshSkipAtlas = 2u16,
    EndStencilMask = 9u16,
    PopClippingRect = 12u16,
    PopDefaultMaterial = 20u16,
    PopGroupMatrix = 16u16,
    PopScissors = 14u16,
    PopStencilMask = 10u16,
    PushClippingRect = 11u16,
    PushDefaultMaterial = 19u16,
    PushGroupMatrix = 15u16,
    PushRenderTexture = 17u16,
    PushScissors = 13u16,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+EntryType")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::UIR::EntryType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "EntryType";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+EntryType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::UIElements::UIR::EntryType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+EntryType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::UIElements::UIR::EntryType {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+EntryType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::UIElements::UIR::EntryType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+EntryType")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::UIElements::UIR::EntryType {
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
