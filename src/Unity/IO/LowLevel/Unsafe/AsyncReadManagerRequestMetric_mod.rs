#[cfg(feature = "cordl_class_Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct AsyncReadManagerRequestMetric {
    pub _AssetName_k__BackingField: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _FileName_k__BackingField: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _OffsetBytes_k__BackingField: u64,
    pub _SizeBytes_k__BackingField: u64,
    pub _AssetTypeId_k__BackingField: u64,
    pub _CurrentBytesRead_k__BackingField: u64,
    pub _BatchReadCount_k__BackingField: u32,
    pub _IsBatchRead_k__BackingField: bool,
    pub _State_k__BackingField: crate::Unity::IO::LowLevel::Unsafe::ProcessingState,
    pub _ReadType_k__BackingField: crate::Unity::IO::LowLevel::Unsafe::FileReadType,
    pub _PriorityLevel_k__BackingField: crate::Unity::IO::LowLevel::Unsafe::Priority,
    pub _Subsystem_k__BackingField: crate::Unity::IO::LowLevel::Unsafe::AssetLoadingSubsystem,
    pub _RequestTimeMicroseconds_k__BackingField: f64,
    pub _TimeInQueueMicroseconds_k__BackingField: f64,
    pub _TotalTimeMicroseconds_k__BackingField: f64,
}
#[cfg(feature = "cordl_class_Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerRequestMetric
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.IO.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "AsyncReadManagerRequestMetric";
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
#[cfg(feature = "cordl_class_Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerRequestMetric
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerRequestMetric
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
#[cfg(feature = "cordl_class_Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerRequestMetric
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
#[cfg(feature = "cordl_class_Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerRequestMetric
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
#[cfg(feature = "cordl_class_Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerRequestMetric
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AsyncReadManagerRequestMetric")]
impl crate::Unity::IO::LowLevel::Unsafe::AsyncReadManagerRequestMetric {}
