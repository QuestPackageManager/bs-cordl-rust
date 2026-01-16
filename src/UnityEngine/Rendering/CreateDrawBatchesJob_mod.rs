#[cfg(feature = "cordl_class_UnityEngine+Rendering+CreateDrawBatchesJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateDrawBatchesJob {
    pub implicitInstanceIndices: bool,
    pub instances: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::InstanceHandle,
    >,
    pub rendererData: crate::UnityEngine::Rendering::GPUDrivenRendererGroupData,
    pub batchMeshHash: crate::Unity::Collections::NativeParallelHashMap_2_ReadOnly<
        i32,
        crate::UnityEngine::Rendering::BatchMeshID,
    >,
    pub batchMaterialHash: crate::Unity::Collections::NativeParallelHashMap_2_ReadOnly<
        i32,
        crate::UnityEngine::Rendering::BatchMaterialID,
    >,
    pub packedMaterialDataHash: crate::Unity::Collections::NativeParallelHashMap_2_ReadOnly<
        i32,
        crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
    >,
    pub rangeHash: crate::Unity::Collections::NativeParallelHashMap_2<
        crate::UnityEngine::Rendering::RangeKey,
        i32,
    >,
    pub drawRanges: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::DrawRange,
    >,
    pub batchHash: crate::Unity::Collections::NativeParallelHashMap_2<
        crate::UnityEngine::Rendering::DrawKey,
        i32,
    >,
    pub drawBatches: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::DrawBatch,
    >,
    pub drawInstances: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::DrawInstance,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CreateDrawBatchesJob")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CreateDrawBatchesJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CreateDrawBatchesJob";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CreateDrawBatchesJob")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::CreateDrawBatchesJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CreateDrawBatchesJob")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::CreateDrawBatchesJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CreateDrawBatchesJob")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::CreateDrawBatchesJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CreateDrawBatchesJob")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::CreateDrawBatchesJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CreateDrawBatchesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::CreateDrawBatchesJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CreateDrawBatchesJob")]
impl crate::UnityEngine::Rendering::CreateDrawBatchesJob {
    pub fn EditDrawBatch(
        &mut self,
        key: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::DrawKey>,
        subMeshDescriptor: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SubMeshDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::DrawBatch>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::DrawKey,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::SubMeshDescriptor,
                            >,
                        ),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::DrawBatch,
                        >,
                        2usize,
                    >("EditDrawBatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EditDrawBatch", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::DrawBatch,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (key, subMeshDescriptor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EditDrawRange(
        &mut self,
        key: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RangeKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::DrawRange>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RangeKey,
                        >),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::DrawRange,
                        >,
                        1usize,
                    >("EditDrawRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EditDrawRange", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::DrawRange,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (key))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessRenderer(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ProcessRenderer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessRenderer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (i))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+CreateDrawBatchesJob")]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::CreateDrawBatchesJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+CreateDrawBatchesJob")]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::CreateDrawBatchesJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
