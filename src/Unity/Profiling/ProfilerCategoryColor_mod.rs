#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerCategoryColor")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProfilerCategoryColor {
    #[default]
    Animation = 5u16,
    Audio = 6u16,
    AudioJob = 7u16,
    AudioUpdateJob = 8u16,
    Build = 15u16,
    BurstJobs = 2u16,
    GC = 10u16,
    Input = 16u16,
    Internal = 13u16,
    Lighting = 9u16,
    Memory = 12u16,
    Other = 3u16,
    Physics = 4u16,
    Render = 0u16,
    Scripts = 1u16,
    UI = 14u16,
    VSync = 11u16,
}
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerCategoryColor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Profiling::ProfilerCategoryColor {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Profiling";
    const CLASS_NAME: &'static str = "ProfilerCategoryColor";
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
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerCategoryColor")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Profiling::ProfilerCategoryColor {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerCategoryColor")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Profiling::ProfilerCategoryColor {
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
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerCategoryColor")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Profiling::ProfilerCategoryColor {
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
#[cfg(feature = "cordl_class_Unity+Profiling+ProfilerCategoryColor")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Profiling::ProfilerCategoryColor {
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
