#[cfg(feature = "UnityEngine+Rendering+CameraEvent")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraEvent {
    #[default]
    AfterDepthNormalsTexture = 3i32,
    AfterDepthTexture = 1i32,
    AfterEverything = 20i32,
    AfterFinalPass = 9i32,
    AfterForwardAlpha = 17i32,
    AfterForwardOpaque = 11i32,
    AfterGBuffer = 5i32,
    AfterHaloAndLensFlares = 24i32,
    AfterImageEffects = 19i32,
    AfterImageEffectsOpaque = 13i32,
    AfterLighting = 7i32,
    AfterReflections = 22i32,
    AfterSkybox = 15i32,
    BeforeDepthNormalsTexture = 2i32,
    BeforeDepthTexture = 0i32,
    BeforeFinalPass = 8i32,
    BeforeForwardAlpha = 16i32,
    BeforeForwardOpaque = 10i32,
    BeforeGBuffer = 4i32,
    BeforeHaloAndLensFlares = 23i32,
    BeforeImageEffects = 18i32,
    BeforeImageEffectsOpaque = 12i32,
    BeforeLighting = 6i32,
    BeforeReflections = 21i32,
    BeforeSkybox = 14i32,
}
#[cfg(feature = "UnityEngine+Rendering+CameraEvent")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::CameraEvent {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CameraEvent";
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
#[cfg(feature = "UnityEngine+Rendering+CameraEvent")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::CameraEvent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Rendering+CameraEvent")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::CameraEvent {
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
#[cfg(feature = "UnityEngine+Rendering+CameraEvent")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::CameraEvent {
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
#[cfg(feature = "UnityEngine+Rendering+CameraEvent")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::CameraEvent {
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
