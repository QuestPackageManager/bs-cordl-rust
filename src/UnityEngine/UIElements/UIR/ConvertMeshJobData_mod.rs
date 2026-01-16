#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+ConvertMeshJobData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConvertMeshJobData {
    pub vertSrc: crate::System::IntPtr,
    pub vertDst: crate::System::IntPtr,
    pub vertCount: i32,
    pub transform: crate::UnityEngine::Matrix4x4,
    pub xformClipPages: crate::UnityEngine::Color32,
    pub ids: crate::UnityEngine::Color32,
    pub addFlags: crate::UnityEngine::Color32,
    pub opacityPage: crate::UnityEngine::Color32,
    pub textCoreSettingsPage: crate::UnityEngine::Color32,
    pub usesTextCoreSettings: i32,
    pub textureId: f32,
    pub gradientSettingsIndexOffset: i32,
    pub indexSrc: crate::System::IntPtr,
    pub indexDst: crate::System::IntPtr,
    pub indexCount: i32,
    pub indexOffset: i32,
    pub flipIndices: i32,
    pub forceZ: i32,
    pub positionZ: f32,
    pub remapUVs: i32,
    pub atlasRect: crate::UnityEngine::Rect,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+ConvertMeshJobData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::ConvertMeshJobData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "ConvertMeshJobData";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+ConvertMeshJobData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::ConvertMeshJobData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+ConvertMeshJobData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::ConvertMeshJobData {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+ConvertMeshJobData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::ConvertMeshJobData {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+ConvertMeshJobData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::ConvertMeshJobData {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+ConvertMeshJobData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::ConvertMeshJobData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ConvertMeshJobData")]
impl crate::UnityEngine::UIElements::UIR::ConvertMeshJobData {}
