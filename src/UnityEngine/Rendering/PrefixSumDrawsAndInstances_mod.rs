#[cfg(feature = "cordl_class_UnityEngine+Rendering+PrefixSumDrawsAndInstances")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct PrefixSumDrawsAndInstances {
    pub drawRanges:
        crate::Unity::Collections::NativeList_1<crate::UnityEngine::Rendering::DrawRange>,
    pub drawBatchIndices: crate::Unity::Collections::NativeArray_1<i32>,
    pub batchBinAllocOffsets: crate::Unity::Collections::NativeArray_1<i32>,
    pub batchBinCounts: crate::Unity::Collections::NativeArray_1<i32>,
    pub binVisibleInstanceCounts: crate::Unity::Collections::NativeArray_1<i32>,
    pub batchDrawCommandOffsets: crate::Unity::Collections::NativeArray_1<i32>,
    pub binVisibleInstanceOffsets: crate::Unity::Collections::NativeArray_1<i32>,
    pub cullingOutput: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::BatchCullingOutputDrawCommands,
    >,
    pub indirectBufferLimits: crate::UnityEngine::Rendering::IndirectBufferLimits,
    pub indirectBufferAllocInfo: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::IndirectBufferAllocInfo,
    >,
    pub indirectAllocationCounters: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PrefixSumDrawsAndInstances")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::PrefixSumDrawsAndInstances
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "PrefixSumDrawsAndInstances";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PrefixSumDrawsAndInstances")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::PrefixSumDrawsAndInstances
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PrefixSumDrawsAndInstances")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::PrefixSumDrawsAndInstances
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PrefixSumDrawsAndInstances")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::PrefixSumDrawsAndInstances
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PrefixSumDrawsAndInstances")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::PrefixSumDrawsAndInstances
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PrefixSumDrawsAndInstances")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::PrefixSumDrawsAndInstances
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+PrefixSumDrawsAndInstances")]
impl crate::UnityEngine::Rendering::PrefixSumDrawsAndInstances {
    pub fn Execute(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+PrefixSumDrawsAndInstances")]
impl AsRef<crate::Unity::Jobs::IJob> for crate::UnityEngine::Rendering::PrefixSumDrawsAndInstances {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+PrefixSumDrawsAndInstances")]
impl AsMut<crate::Unity::Jobs::IJob> for crate::UnityEngine::Rendering::PrefixSumDrawsAndInstances {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
