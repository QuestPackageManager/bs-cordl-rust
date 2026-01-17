#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct CullingJob {
    pub binningConfig: crate::UnityEngine::Rendering::BinningConfig,
    pub viewType: crate::UnityEngine::Rendering::BatchCullingViewType,
    pub cameraPosition: crate::Unity::Mathematics::float3,
    pub sqrScreenRelativeMetric: f32,
    pub minScreenRelativeHeight: f32,
    pub isOrtho: bool,
    pub cullLightmappedShadowCasters: bool,
    pub maxLOD: i32,
    pub cullingLayerMask: u32,
    pub sceneCullingMask: u64,
    pub frustumPlanePackets: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4,
    >,
    pub frustumSplitInfos: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo,
    >,
    pub lightFacingFrustumPlanes:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Plane>,
    pub receiverSplitInfos: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo,
    >,
    pub worldToLightSpaceRotation: crate::Unity::Mathematics::float3x3,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
    pub sharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly,
    pub lodGroupCullingData:
        crate::Unity::Collections::NativeList_1<crate::UnityEngine::Rendering::LODGroupCullingData>,
    pub occlusionBuffer: crate::System::IntPtr,
    pub rendererVisibilityMasks: crate::Unity::Collections::NativeArray_1<u8>,
    pub rendererCrossFadeValues: crate::Unity::Collections::NativeArray_1<u8>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::CullingJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CullingJob";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Rendering::CullingJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::Rendering::CullingJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Rendering::CullingJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Rendering::CullingJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Rendering::CullingJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CullingJob")]
impl crate::UnityEngine::Rendering::CullingJob {
    pub const k_BatchSize: i32 = 32i32;
    pub const k_LODFadeZeroPacked: u32 = 127u32;
    pub const k_LODPercentFullyVisible: f32 = 1f32;
    pub const k_LODPercentInvisible: f32 = 0f32;
    pub const k_LODPercentSpeedTree: f32 = 2f32;
    pub const k_SmallMeshTransitionWidth: f32 = 0.1f32;
    #[cfg(feature = "UnityEngine+Rendering+CullingJob+CrossFadeType")]
    pub type CrossFadeType = crate::UnityEngine::Rendering::CullingJob_CrossFadeType;
    pub fn CalculateLODVisibility(
        &mut self,
        instanceIndex: i32,
        sharedInstanceIndex: i32,
        instanceFlags: crate::UnityEngine::Rendering::InstanceFlags,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, crate::UnityEngine::Rendering::InstanceFlags),
                        f32,
                        3usize,
                    >("CalculateLODVisibility")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateLODVisibility", 3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (instanceIndex, sharedInstanceIndex, instanceFlags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateVisibilityMask(
        &mut self,
        instanceIndex: i32,
        sharedInstanceIndex: i32,
        instanceFlags: crate::UnityEngine::Rendering::InstanceFlags,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, crate::UnityEngine::Rendering::InstanceFlags),
                        u32,
                        3usize,
                    >("CalculateVisibilityMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateVisibilityMask", 3usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (instanceIndex, sharedInstanceIndex, instanceFlags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        instanceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instanceIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn PackFloatToUint8(percent: f32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), u32, 1usize>("PackFloatToUint8")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PackFloatToUint8",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (percent))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+CullingJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor> for crate::UnityEngine::Rendering::CullingJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+CullingJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor> for crate::UnityEngine::Rendering::CullingJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob+CrossFadeType")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum CullingJob_CrossFadeType {
    #[cfg_attr(feature = "derive_Default", default)]
    kCrossFadeIn = 2i32,
    kCrossFadeOut = 1i32,
    kDisabled = 0i32,
    kVisible = 3i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob+CrossFadeType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::CullingJob_CrossFadeType
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CullingJob/CrossFadeType";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob+CrossFadeType")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::CullingJob_CrossFadeType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob+CrossFadeType")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::CullingJob_CrossFadeType
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob+CrossFadeType")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::CullingJob_CrossFadeType
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingJob+CrossFadeType")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::CullingJob_CrossFadeType
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
