#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct GPUInstanceDataBufferUploader {
    pub m_UintPerInstance: i32,
    pub m_Capacity: i32,
    pub m_InstanceCount: i32,
    pub m_ComponentIsInstanced: crate::Unity::Collections::NativeArray_1<bool>,
    pub m_ComponentDataIndex: crate::Unity::Collections::NativeArray_1<i32>,
    pub m_DescriptionsUintSize: crate::Unity::Collections::NativeArray_1<i32>,
    pub m_TmpDataBuffer: crate::Unity::Collections::NativeArray_1<u32>,
    pub m_WritenComponentIndices: crate::Unity::Collections::NativeList_1<i32>,
    pub m_DummyArray: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUInstanceDataBufferUploader";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader")]
impl crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader {
    #[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+GPUResources")]
    pub type GPUResources =
        crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources;
    #[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+UploadKernelIDs")]
    pub type UploadKernelIDs =
        crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_UploadKernelIDs;
    #[cfg(
        feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+WriteInstanceDataParameterJob"
    )]
    pub type WriteInstanceDataParameterJob =
        crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_WriteInstanceDataParameterJob;
    pub fn AllocateUploadHandles(
        &mut self,
        handlesLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "AllocateUploadHandles",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AllocateUploadHandles",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (handlesLength))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetParamUIntOffset(
        &mut self,
        parameterIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), i32, 1usize>("GetParamUIntOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetParamUIntOffset",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (parameterIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUIntPerInstance(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetUIntPerInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetUIntPerInstance",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUploadBufferPtr(&mut self) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::IntPtr, 0usize>("GetUploadBufferPtr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetUploadBufferPtr",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareParamWrite<T>(
        &mut self,
        parameterIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), i32, 1usize>("PrepareParamWrite")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PrepareParamWrite",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (parameterIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn SubmitToGpu_GPUInstanceDataBuffer_NativeArray_1_ByRefMut__cordl_bool0(
        &mut self,
        instanceDataBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
        gpuInstanceIndices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::GPUInstanceIndex,
        >,
        gpuResources: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources,
        >,
        submitOnlyWrittenParams: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::GPUInstanceIndex,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SubmitToGpu")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SubmitToGpu", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    instanceDataBuffer,
                    gpuInstanceIndices,
                    gpuResources,
                    submitOnlyWrittenParams,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubmitToGpu_GPUInstanceDataBuffer_NativeArray_1_ByRefMut__cordl_bool1(
        &mut self,
        instanceDataBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        gpuResources: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources,
        >,
        submitOnlyWrittenParams: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SubmitToGpu")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SubmitToGpu", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    instanceDataBuffer,
                    instances,
                    gpuResources,
                    submitOnlyWrittenParams,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteInstanceDataJob_NativeArray_1_1<T>(
        &mut self,
        parameterIndex: i32,
        instanceData: crate::Unity::Collections::NativeArray_1<T>,
        gatherIndices: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::Unity::Collections::NativeArray_1<T>,
                        crate::Unity::Collections::NativeArray_1<i32>,
                    ), crate::Unity::Jobs::JobHandle, 3usize>(
                        "WriteInstanceDataJob"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WriteInstanceDataJob",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (parameterIndex, instanceData, gatherIndices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteInstanceDataJob_i32_NativeArray_1_0<T>(
        &mut self,
        parameterIndex: i32,
        instanceData: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::Unity::Collections::NativeArray_1<T>),
                        crate::Unity::Jobs::JobHandle,
                        2usize,
                    >("WriteInstanceDataJob")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteInstanceDataJob", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (parameterIndex, instanceData))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        descriptions: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<
                crate::UnityEngine::Rendering::GPUInstanceComponentDesc,
            >,
        >,
        capacity: i32,
        instanceType: crate::UnityEngine::Rendering::InstanceType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::GPUInstanceComponentDesc,
                            >,
                        >,
                        i32,
                        crate::UnityEngine::Rendering::InstanceType,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (descriptions, capacity, instanceType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+GPUResources")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct GPUInstanceDataBufferUploader_GPUResources {
    pub instanceData: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub instanceIndices: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub inputComponentOffsets: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub validComponentIndices: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub cs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub kernelId: i32,
    pub m_InstanceDataByteSize: i32,
    pub m_InstanceCount: i32,
    pub m_ComponentCounts: i32,
    pub m_ValidComponentIndicesCount: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+GPUResources")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUInstanceDataBufferUploader/GPUResources";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+GPUResources")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+GPUResources")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+GPUResources")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+GPUResources")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+GPUResources")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+GPUResources")]
impl crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources {
    pub fn CreateResources(
        &mut self,
        newInstanceCount: i32,
        sizePerInstance: i32,
        newComponentCounts: i32,
        validComponentIndicesCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32, i32, i32), quest_hook::libil2cpp::Void, 4usize>(
                        "CreateResources",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateResources",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    newInstanceCount,
                    sizePerInstance,
                    newComponentCounts,
                    validComponentIndicesCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadShaders(
        &mut self,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::GPUResidentDrawerResources,
                    >), quest_hook::libil2cpp::Void, 1usize>("LoadShaders")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadShaders",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (resources))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+GPUResources")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+GPUResources")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+UploadKernelIDs")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct GPUInstanceDataBufferUploader_UploadKernelIDs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+UploadKernelIDs")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_UploadKernelIDs
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUInstanceDataBufferUploader/UploadKernelIDs";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+UploadKernelIDs")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_UploadKernelIDs
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+UploadKernelIDs")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_UploadKernelIDs
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+UploadKernelIDs")]
impl crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_UploadKernelIDs {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+UploadKernelIDs")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_UploadKernelIDs
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+WriteInstanceDataParameterJob"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct GPUInstanceDataBufferUploader_WriteInstanceDataParameterJob {
    pub gatherData: bool,
    pub parameterIndex: i32,
    pub uintPerParameter: i32,
    pub uintPerInstance: i32,
    pub componentDataIndex: crate::Unity::Collections::NativeArray_1<i32>,
    pub gatherIndices: crate::Unity::Collections::NativeArray_1<i32>,
    pub instanceData: crate::Unity::Collections::NativeArray_1<u32>,
    pub tmpDataBuffer: crate::Unity::Collections::NativeArray_1<u32>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+WriteInstanceDataParameterJob"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_WriteInstanceDataParameterJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUInstanceDataBufferUploader/WriteInstanceDataParameterJob";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+WriteInstanceDataParameterJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_WriteInstanceDataParameterJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+WriteInstanceDataParameterJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_WriteInstanceDataParameterJob
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+WriteInstanceDataParameterJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_WriteInstanceDataParameterJob
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+WriteInstanceDataParameterJob"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_WriteInstanceDataParameterJob
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUInstanceDataBufferUploader+WriteInstanceDataParameterJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_WriteInstanceDataParameterJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+WriteInstanceDataParameterJob"
)]
impl crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_WriteInstanceDataParameterJob {
    pub const k_BatchSize: i32 = 512i32;
    pub fn Execute(
        &mut self,
        index: i32,
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
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+WriteInstanceDataParameterJob"
)]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_WriteInstanceDataParameterJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+GPUInstanceDataBufferUploader+WriteInstanceDataParameterJob"
)]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_WriteInstanceDataParameterJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
