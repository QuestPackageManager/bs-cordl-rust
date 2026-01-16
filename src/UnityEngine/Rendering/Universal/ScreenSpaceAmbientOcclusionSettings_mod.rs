#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenSpaceAmbientOcclusionSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub AOMethod: crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOMethodOptions,
    pub Downsample: bool,
    pub AfterOpaque: bool,
    pub Source: crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_DepthSource,
    pub NormalSamples: crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_NormalQuality,
    pub Intensity: f32,
    pub DirectLightingStrength: f32,
    pub Radius: f32,
    pub Samples: crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOSampleOption,
    pub BlurQuality: crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_BlurQualityOptions,
    pub Falloff: f32,
    pub SampleCount: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceAmbientOcclusionSettings";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings")]
impl crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings {
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOMethodOptions"
    )]
    pub type AOMethodOptions = crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOMethodOptions;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOSampleOption"
    )]
    pub type AOSampleOption = crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOSampleOption;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+BlurQualityOptions"
    )]
    pub type BlurQualityOptions = crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_BlurQualityOptions;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+DepthSource"
    )]
    pub type DepthSource = crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_DepthSource;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+NormalQuality"
    )]
    pub type NormalQuality = crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_NormalQuality;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOMethodOptions"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenSpaceAmbientOcclusionSettings_AOMethodOptions {
    #[default]
    BlueNoise = 0i32,
    InterleavedGradient = 1i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOMethodOptions"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOMethodOptions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceAmbientOcclusionSettings/AOMethodOptions";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOMethodOptions"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOMethodOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOMethodOptions"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOMethodOptions {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOMethodOptions"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOMethodOptions {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOMethodOptions"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOMethodOptions {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOSampleOption"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenSpaceAmbientOcclusionSettings_AOSampleOption {
    #[default]
    High = 0i32,
    Low = 2i32,
    Medium = 1i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOSampleOption"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOSampleOption {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceAmbientOcclusionSettings/AOSampleOption";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOSampleOption"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOSampleOption {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOSampleOption"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOSampleOption {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOSampleOption"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOSampleOption {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+AOSampleOption"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_AOSampleOption {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+BlurQualityOptions"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenSpaceAmbientOcclusionSettings_BlurQualityOptions {
    #[default]
    High = 0i32,
    Low = 2i32,
    Medium = 1i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+BlurQualityOptions"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_BlurQualityOptions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceAmbientOcclusionSettings/BlurQualityOptions";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+BlurQualityOptions"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_BlurQualityOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+BlurQualityOptions"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_BlurQualityOptions {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+BlurQualityOptions"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_BlurQualityOptions {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+BlurQualityOptions"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_BlurQualityOptions {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+DepthSource"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenSpaceAmbientOcclusionSettings_DepthSource {
    #[default]
    Depth = 0i32,
    DepthNormals = 1i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+DepthSource"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_DepthSource {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceAmbientOcclusionSettings/DepthSource";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+DepthSource"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_DepthSource {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+DepthSource"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_DepthSource {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+DepthSource"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_DepthSource {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+DepthSource"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_DepthSource {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+NormalQuality"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenSpaceAmbientOcclusionSettings_NormalQuality {
    #[default]
    High = 2i32,
    Low = 0i32,
    Medium = 1i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+NormalQuality"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_NormalQuality {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceAmbientOcclusionSettings/NormalQuality";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+NormalQuality"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_NormalQuality {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+NormalQuality"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_NormalQuality {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+NormalQuality"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_NormalQuality {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionSettings+NormalQuality"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_NormalQuality {
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
