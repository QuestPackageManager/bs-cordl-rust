#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusSettings {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_StereoRenderingModeDesktop: crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeDesktop,
    pub m_StereoRenderingModeAndroid: crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeAndroid,
    pub SharedDepthBuffer: bool,
    pub DepthSubmission: bool,
    pub DashSupport: bool,
    pub LowOverheadMode: bool,
    pub OptimizeBufferDiscards: bool,
    pub PhaseSync: bool,
    pub SymmetricProjection: bool,
    pub SubsampledLayout: bool,
    pub FoveatedRenderingMethod: crate::Unity::XR::Oculus::OculusSettings_FoveationMethod,
    pub LateLatching: bool,
    pub LateLatchingDebug: bool,
    pub EnableTrackingOriginStageMode: bool,
    pub SpaceWarp: bool,
    pub TargetQuest: bool,
    pub TargetQuest2: bool,
    pub TargetQuestPro: bool,
    pub SystemSplashScreen: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::XR::Oculus::OculusSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "OculusSettings";
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
impl std::ops::Deref for crate::Unity::XR::Oculus::OculusSettings {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::OculusSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
impl crate::Unity::XR::Oculus::OculusSettings {
    #[cfg(feature = "Unity+XR+Oculus+OculusSettings+FoveationMethod")]
    pub type FoveationMethod = crate::Unity::XR::Oculus::OculusSettings_FoveationMethod;
    #[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeAndroid")]
    pub type StereoRenderingModeAndroid = crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeAndroid;
    #[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeDesktop")]
    pub type StereoRenderingModeDesktop = crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeDesktop;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Awake",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoRenderingMode(&mut self) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u16, 0usize>("GetStereoRenderingMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetStereoRenderingMode", 0usize
                        )
                    })
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
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
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::OculusSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+FoveationMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OculusSettings_FoveationMethod {
    #[default]
    EyeTrackedFoveatedRendering = 1i32,
    FixedFoveatedRendering = 0i32,
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+FoveationMethod")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::XR::Oculus::OculusSettings_FoveationMethod {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "OculusSettings/FoveationMethod";
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+FoveationMethod")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::XR::Oculus::OculusSettings_FoveationMethod {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+FoveationMethod")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::XR::Oculus::OculusSettings_FoveationMethod {
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+FoveationMethod")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::XR::Oculus::OculusSettings_FoveationMethod {
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+FoveationMethod")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::XR::Oculus::OculusSettings_FoveationMethod {
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeAndroid")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OculusSettings_StereoRenderingModeAndroid {
    #[default]
    MultiPass = 0i32,
    Multiview = 2i32,
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeAndroid")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeAndroid {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "OculusSettings/StereoRenderingModeAndroid";
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeAndroid")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeAndroid {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeAndroid")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeAndroid {
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeAndroid")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeAndroid {
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeAndroid")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeAndroid {
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeDesktop")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OculusSettings_StereoRenderingModeDesktop {
    #[default]
    MultiPass = 0i32,
    SinglePassInstanced = 1i32,
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeDesktop")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeDesktop {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "OculusSettings/StereoRenderingModeDesktop";
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeDesktop")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeDesktop {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeDesktop")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeDesktop {
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeDesktop")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeDesktop {
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
#[cfg(feature = "Unity+XR+Oculus+OculusSettings+StereoRenderingModeDesktop")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::XR::Oculus::OculusSettings_StereoRenderingModeDesktop {
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
