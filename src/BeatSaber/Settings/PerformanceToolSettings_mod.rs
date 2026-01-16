#[cfg(feature = "cordl_class_BeatSaber+Settings+PerformanceToolSettings")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PerformanceToolSettings {
    pub recordFps: bool,
    pub showFps: bool,
    pub showMemory: bool,
    pub saveScreenshot: bool,
}
#[cfg(feature = "cordl_class_BeatSaber+Settings+PerformanceToolSettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Settings::PerformanceToolSettings {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "PerformanceToolSettings";
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+PerformanceToolSettings")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::BeatSaber::Settings::PerformanceToolSettings
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Settings+PerformanceToolSettings")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::BeatSaber::Settings::PerformanceToolSettings
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+PerformanceToolSettings")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::BeatSaber::Settings::PerformanceToolSettings
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+PerformanceToolSettings")]
unsafe impl quest_hook::libil2cpp::Return for crate::BeatSaber::Settings::PerformanceToolSettings {
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
#[cfg(feature = "cordl_class_BeatSaber+Settings+PerformanceToolSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::BeatSaber::Settings::PerformanceToolSettings
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+PerformanceToolSettings")]
impl crate::BeatSaber::Settings::PerformanceToolSettings {}
