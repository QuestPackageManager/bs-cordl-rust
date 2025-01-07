#[cfg(feature = "Unity+Profiling+LowLevel+ProfilerMarkerDataType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProfilerMarkerDataType {
    #[default]
    Blob8 = 11u8,
    Double = 7u8,
    Float = 6u8,
    GfxResourceId = 12u8,
    InstanceId = 1u8,
    Int32 = 2u8,
    Int64 = 4u8,
    String16 = 9u8,
    UInt32 = 3u8,
    UInt64 = 5u8,
}
#[cfg(feature = "Unity+Profiling+LowLevel+ProfilerMarkerDataType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Profiling::LowLevel::ProfilerMarkerDataType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Profiling.LowLevel";
    const CLASS_NAME: &'static str = "ProfilerMarkerDataType";
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
#[cfg(feature = "Unity+Profiling+LowLevel+ProfilerMarkerDataType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Profiling::LowLevel::ProfilerMarkerDataType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+ProfilerMarkerDataType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Profiling::LowLevel::ProfilerMarkerDataType {
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
#[cfg(feature = "Unity+Profiling+LowLevel+ProfilerMarkerDataType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Profiling::LowLevel::ProfilerMarkerDataType {
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
#[cfg(feature = "Unity+Profiling+LowLevel+ProfilerMarkerDataType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Profiling::LowLevel::ProfilerMarkerDataType {
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
