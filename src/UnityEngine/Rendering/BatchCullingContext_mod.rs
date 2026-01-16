#[cfg(feature = "cordl_class_UnityEngine+Rendering+BatchCullingContext")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct BatchCullingContext {
    pub cullingPlanes: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Plane>,
    pub cullingSplits:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::CullingSplit>,
    pub lodParameters: crate::UnityEngine::Rendering::LODParameters,
    pub localToWorldMatrix: crate::UnityEngine::Matrix4x4,
    pub viewType: crate::UnityEngine::Rendering::BatchCullingViewType,
    pub projectionType: crate::UnityEngine::Rendering::BatchCullingProjectionType,
    pub cullingFlags: crate::UnityEngine::Rendering::BatchCullingFlags,
    pub viewID: crate::UnityEngine::Rendering::BatchPackedCullingViewID,
    pub cullingLayerMask: u32,
    pub sceneCullingMask: u64,
    pub splitExclusionMask: u16,
    pub isOrthographic: u8,
    pub receiverPlaneOffset: i32,
    pub receiverPlaneCount: i32,
    pub occlusionBuffer: crate::System::IntPtr,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BatchCullingContext")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::BatchCullingContext {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "BatchCullingContext";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BatchCullingContext")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Rendering::BatchCullingContext {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BatchCullingContext")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::BatchCullingContext
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BatchCullingContext")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Rendering::BatchCullingContext {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BatchCullingContext")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Rendering::BatchCullingContext {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BatchCullingContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::BatchCullingContext
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchCullingContext")]
impl crate::UnityEngine::Rendering::BatchCullingContext {
    pub fn _ctor(
        &mut self,
        inCullingPlanes: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Plane>,
        inCullingSplits: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::CullingSplit,
        >,
        inLodParameters: crate::UnityEngine::Rendering::LODParameters,
        inLocalToWorldMatrix: crate::UnityEngine::Matrix4x4,
        inViewType: crate::UnityEngine::Rendering::BatchCullingViewType,
        inProjectionType: crate::UnityEngine::Rendering::BatchCullingProjectionType,
        inBatchCullingFlags: crate::UnityEngine::Rendering::BatchCullingFlags,
        inViewID: u64,
        inCullingLayerMask: u32,
        inSceneCullingMask: u64,
        inExclusionSplitMask: u8,
        inReceiverPlaneOffset: i32,
        inReceiverPlaneCount: i32,
        inOcclusionBuffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Plane>,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::CullingSplit,
                        >,
                        crate::UnityEngine::Rendering::LODParameters,
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Rendering::BatchCullingViewType,
                        crate::UnityEngine::Rendering::BatchCullingProjectionType,
                        crate::UnityEngine::Rendering::BatchCullingFlags,
                        u64,
                        u32,
                        u64,
                        u8,
                        i32,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 14usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            14usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    inCullingPlanes,
                    inCullingSplits,
                    inLodParameters,
                    inLocalToWorldMatrix,
                    inViewType,
                    inProjectionType,
                    inBatchCullingFlags,
                    inViewID,
                    inCullingLayerMask,
                    inSceneCullingMask,
                    inExclusionSplitMask,
                    inReceiverPlaneOffset,
                    inReceiverPlaneCount,
                    inOcclusionBuffer,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
