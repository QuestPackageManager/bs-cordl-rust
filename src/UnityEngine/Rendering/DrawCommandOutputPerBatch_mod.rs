#[cfg(feature = "cordl_class_UnityEngine+Rendering+DrawCommandOutputPerBatch")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DrawCommandOutputPerBatch {
    pub binningConfig: crate::UnityEngine::Rendering::BinningConfig,
    pub batchIDs: crate::Unity::Collections::NativeParallelHashMap_2<
        u32,
        crate::UnityEngine::Rendering::BatchID,
    >,
    pub instanceDataBuffer: crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly,
    pub drawBatches: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::DrawBatch,
    >,
    pub drawInstanceIndices: crate::Unity::Collections::NativeArray_1<i32>,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
    pub rendererVisibilityMasks: crate::Unity::Collections::NativeArray_1<u8>,
    pub rendererCrossFadeValues: crate::Unity::Collections::NativeArray_1<u8>,
    pub batchBinAllocOffsets: crate::Unity::Collections::NativeArray_1<i32>,
    pub batchBinCounts: crate::Unity::Collections::NativeArray_1<i32>,
    pub batchDrawCommandOffsets: crate::Unity::Collections::NativeArray_1<i32>,
    pub binConfigIndices: crate::Unity::Collections::NativeArray_1<i16>,
    pub binVisibleInstanceOffsets: crate::Unity::Collections::NativeArray_1<i32>,
    pub binVisibleInstanceCounts: crate::Unity::Collections::NativeArray_1<i32>,
    pub cullingOutput: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::BatchCullingOutputDrawCommands,
    >,
    pub indirectBufferLimits: crate::UnityEngine::Rendering::IndirectBufferLimits,
    pub visibleInstancesBufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    pub indirectArgsBufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    pub indirectBufferAllocInfo: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::IndirectBufferAllocInfo,
    >,
    pub indirectDrawInfoGlobalArray: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::IndirectDrawInfo,
    >,
    pub indirectInstanceInfoGlobalArray: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::IndirectInstanceInfo,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DrawCommandOutputPerBatch")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::DrawCommandOutputPerBatch {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "DrawCommandOutputPerBatch";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DrawCommandOutputPerBatch")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::DrawCommandOutputPerBatch {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DrawCommandOutputPerBatch")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::DrawCommandOutputPerBatch {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DrawCommandOutputPerBatch")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::DrawCommandOutputPerBatch {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DrawCommandOutputPerBatch")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::DrawCommandOutputPerBatch {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DrawCommandOutputPerBatch")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::DrawCommandOutputPerBatch {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DrawCommandOutputPerBatch")]
impl crate::UnityEngine::Rendering::DrawCommandOutputPerBatch {
    pub fn EncodeGPUInstanceIndexAndCrossFade(
        &mut self,
        rendererIndex: i32,
        negateCrossFade: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, bool),
                        i32,
                        2usize,
                    >("EncodeGPUInstanceIndexAndCrossFade")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EncodeGPUInstanceIndexAndCrossFade", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (rendererIndex, negateCrossFade))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        batchIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (batchIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsInstanceFlipped(
        &mut self,
        rendererIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), bool, 1usize>("IsInstanceFlipped")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsInstanceFlipped", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (rendererIndex))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+DrawCommandOutputPerBatch")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
for crate::UnityEngine::Rendering::DrawCommandOutputPerBatch {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+DrawCommandOutputPerBatch")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
for crate::UnityEngine::Rendering::DrawCommandOutputPerBatch {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
