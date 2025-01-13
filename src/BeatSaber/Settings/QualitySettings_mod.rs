#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct QualitySettings {
    pub renderViewportScale: f32,
    pub vrResolutionScale: f32,
    pub menuVRResolutionScaleMultiplier: f32,
    pub antiAliasingLevel: i32,
    pub targetFramerate: i32,
    pub vSyncCount: i32,
    pub maxQueuedFrames: i32,
    pub mainEffect: crate::BeatSaber::Settings::QualitySettings_MainEffectOption,
    pub bloom: crate::BeatSaber::Settings::QualitySettings_BloomQuality,
    pub mirror: crate::BeatSaber::Settings::QualitySettings_MirrorQuality,
    pub obstacles: crate::BeatSaber::Settings::QualitySettings_ObstacleQuality,
    pub screenDisplacementEffects: bool,
    pub smokeGraphics: bool,
    pub depthTexture: bool,
    pub burnMarkTrails: bool,
    pub maxShockwaveParticles: i32,
    pub maxNumberOfCutSoundEffects: i32,
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Settings::QualitySettings {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "QualitySettings";
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::Settings::QualitySettings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::Settings::QualitySettings {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::Settings::QualitySettings {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::Settings::QualitySettings {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::QualitySettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
impl crate::BeatSaber::Settings::QualitySettings {
    #[cfg(feature = "BeatSaber+Settings+QualitySettings+BloomQuality")]
    pub type BloomQuality = crate::BeatSaber::Settings::QualitySettings_BloomQuality;
    #[cfg(feature = "BeatSaber+Settings+QualitySettings+MainEffectOption")]
    pub type MainEffectOption = crate::BeatSaber::Settings::QualitySettings_MainEffectOption;
    #[cfg(feature = "BeatSaber+Settings+QualitySettings+MirrorQuality")]
    pub type MirrorQuality = crate::BeatSaber::Settings::QualitySettings_MirrorQuality;
    #[cfg(feature = "BeatSaber+Settings+QualitySettings+ObstacleQuality")]
    pub type ObstacleQuality = crate::BeatSaber::Settings::QualitySettings_ObstacleQuality;
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+BloomQuality")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QualitySettings_BloomQuality {
    #[default]
    Game = 0i32,
    LightBaking = 1i32,
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+BloomQuality")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Settings::QualitySettings_BloomQuality {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "QualitySettings/BloomQuality";
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+BloomQuality")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::Settings::QualitySettings_BloomQuality {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+BloomQuality")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::Settings::QualitySettings_BloomQuality {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+BloomQuality")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::Settings::QualitySettings_BloomQuality {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+BloomQuality")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::Settings::QualitySettings_BloomQuality {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MainEffectOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QualitySettings_MainEffectOption {
    #[default]
    Game = 1i32,
    LightBaking = 2i32,
    Off = 0i32,
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MainEffectOption")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Settings::QualitySettings_MainEffectOption {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "QualitySettings/MainEffectOption";
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MainEffectOption")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::Settings::QualitySettings_MainEffectOption {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MainEffectOption")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::Settings::QualitySettings_MainEffectOption {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MainEffectOption")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::Settings::QualitySettings_MainEffectOption {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MainEffectOption")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::Settings::QualitySettings_MainEffectOption {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MirrorQuality")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QualitySettings_MirrorQuality {
    #[default]
    High = 3i32,
    Low = 1i32,
    Medium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MirrorQuality")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Settings::QualitySettings_MirrorQuality {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "QualitySettings/MirrorQuality";
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MirrorQuality")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::Settings::QualitySettings_MirrorQuality {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MirrorQuality")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::Settings::QualitySettings_MirrorQuality {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MirrorQuality")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::Settings::QualitySettings_MirrorQuality {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MirrorQuality")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::Settings::QualitySettings_MirrorQuality {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+ObstacleQuality")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QualitySettings_ObstacleQuality {
    #[default]
    High = 2i32,
    Low = 0i32,
    Medium = 1i32,
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+ObstacleQuality")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Settings::QualitySettings_ObstacleQuality {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "QualitySettings/ObstacleQuality";
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+ObstacleQuality")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::Settings::QualitySettings_ObstacleQuality {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+ObstacleQuality")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::Settings::QualitySettings_ObstacleQuality {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+ObstacleQuality")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::Settings::QualitySettings_ObstacleQuality {
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
#[cfg(feature = "BeatSaber+Settings+QualitySettings+ObstacleQuality")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::Settings::QualitySettings_ObstacleQuality {
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
