#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct QuestSettings {
    pub cpuLevel: crate::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel,
    pub gpuLevel: crate::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel,
    pub foveatedRenderingMenu: crate::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel,
    pub foveatedRenderingGameplay: crate::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel,
    pub eyeTrackedFoveatedRendering: bool,
    pub dynamicFoveatedRendering: bool,
}
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Settings::QuestSettings {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "QuestSettings";
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings")]
unsafe impl quest_hook::libil2cpp::Argument for crate::BeatSaber::Settings::QuestSettings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::BeatSaber::Settings::QuestSettings {
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings")]
unsafe impl quest_hook::libil2cpp::Returned for crate::BeatSaber::Settings::QuestSettings {
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings")]
unsafe impl quest_hook::libil2cpp::Return for crate::BeatSaber::Settings::QuestSettings {
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::BeatSaber::Settings::QuestSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+QuestSettings")]
impl crate::BeatSaber::Settings::QuestSettings {
    #[cfg(feature = "BeatSaber+Settings+QuestSettings+FoveatedRenderingLevel")]
    pub type FoveatedRenderingLevel =
        crate::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel;
    #[cfg(feature = "BeatSaber+Settings+QuestSettings+SuggestedPerformanceLevel")]
    pub type SuggestedPerformanceLevel =
        crate::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel;
}
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+FoveatedRenderingLevel")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum QuestSettings_FoveatedRenderingLevel {
    #[cfg_attr(feature = "derive_Default", default)]
    High = 3i32,
    HighTop = 4i32,
    Low = 1i32,
    Medium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+FoveatedRenderingLevel")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "QuestSettings/FoveatedRenderingLevel";
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+FoveatedRenderingLevel")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+FoveatedRenderingLevel")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel
{
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+FoveatedRenderingLevel")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel
{
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+FoveatedRenderingLevel")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel
{
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+SuggestedPerformanceLevel")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum QuestSettings_SuggestedPerformanceLevel {
    #[cfg_attr(feature = "derive_Default", default)]
    Boost = 4i32,
    Default = 0i32,
    PowerSavings = 1i32,
    SustainedHigh = 3i32,
    SustainedLow = 2i32,
}
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+SuggestedPerformanceLevel")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "QuestSettings/SuggestedPerformanceLevel";
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+SuggestedPerformanceLevel")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+SuggestedPerformanceLevel")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel
{
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+SuggestedPerformanceLevel")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel
{
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+QuestSettings+SuggestedPerformanceLevel")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel
{
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
