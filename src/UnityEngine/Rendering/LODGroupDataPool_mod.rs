#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupDataPool")]
#[repr(C)]
#[derive(Debug)]
pub struct LODGroupDataPool {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_LODGroupData:
        crate::Unity::Collections::NativeList_1<crate::UnityEngine::Rendering::LODGroupData>,
    pub m_LODGroupDataHash: crate::Unity::Collections::NativeParallelHashMap_2<
        i32,
        crate::UnityEngine::Rendering::GPUInstanceIndex,
    >,
    pub m_LODGroupCullingData:
        crate::Unity::Collections::NativeList_1<crate::UnityEngine::Rendering::LODGroupCullingData>,
    pub m_FreeLODGroupDataHandles:
        crate::Unity::Collections::NativeList_1<crate::UnityEngine::Rendering::GPUInstanceIndex>,
    pub m_CrossfadedRendererCount: i32,
    pub m_SupportDitheringCrossFade: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupDataPool")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::LODGroupDataPool {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "LODGroupDataPool";
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
#[cfg(feature = "UnityEngine+Rendering+LODGroupDataPool")]
impl std::ops::Deref for crate::UnityEngine::Rendering::LODGroupDataPool {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LODGroupDataPool")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::LODGroupDataPool {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LODGroupDataPool")]
impl crate::UnityEngine::Rendering::LODGroupDataPool {
    #[cfg(feature = "UnityEngine+Rendering+LODGroupDataPool+LodGroupShaderIDs")]
    pub type LodGroupShaderIDs = crate::UnityEngine::Rendering::LODGroupDataPool_LodGroupShaderIDs;
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
    pub fn FreeLODGroupData(
        &mut self,
        destroyedLODGroupsID: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<i32>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FreeLODGroupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FreeLODGroupData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (destroyedLODGroupsID))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        >,
        initialInstanceCount: i32,
        supportDitheringCrossFade: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (resources, initialInstanceCount, supportDitheringCrossFade),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateLODGroupData(
        &mut self,
        inputData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUDrivenLODGroupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::GPUDrivenLODGroupData,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UpdateLODGroupData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateLODGroupData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (inputData))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLODGroupTransformData(
        &mut self,
        inputData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUDrivenLODGroupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::GPUDrivenLODGroupData,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UpdateLODGroupTransformData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateLODGroupTransformData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (inputData))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        >,
        initialInstanceCount: i32,
        supportDitheringCrossFade: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
                        >,
                        i32,
                        bool,
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
            cordl_method_info.invoke_unchecked(
                self,
                (resources, initialInstanceCount, supportDitheringCrossFade),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_activeLodGroupCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_activeLodGroupCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_activeLodGroupCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_crossfadedRendererCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_crossfadedRendererCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_crossfadedRendererCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_lodGroupCullingData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeList_1<crate::UnityEngine::Rendering::LODGroupCullingData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::NativeList_1<
                        crate::UnityEngine::Rendering::LODGroupCullingData,
                    >, 0usize>("get_lodGroupCullingData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_lodGroupCullingData",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeList_1<
            crate::UnityEngine::Rendering::LODGroupCullingData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_lodGroupDataHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeParallelHashMap_2<
            i32,
            crate::UnityEngine::Rendering::GPUInstanceIndex,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::NativeParallelHashMap_2<
                        i32,
                        crate::UnityEngine::Rendering::GPUInstanceIndex,
                    >, 0usize>("get_lodGroupDataHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_lodGroupDataHash",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeParallelHashMap_2<
            i32,
            crate::UnityEngine::Rendering::GPUInstanceIndex,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupDataPool")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::LODGroupDataPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+LODGroupDataPool")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::Rendering::LODGroupDataPool {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LODGroupDataPool")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::Rendering::LODGroupDataPool {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupDataPool+LodGroupShaderIDs")]
#[repr(C)]
#[derive(Debug)]
pub struct LODGroupDataPool_LodGroupShaderIDs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupDataPool+LodGroupShaderIDs")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::LODGroupDataPool_LodGroupShaderIDs
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "LODGroupDataPool/LodGroupShaderIDs";
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
#[cfg(feature = "UnityEngine+Rendering+LODGroupDataPool+LodGroupShaderIDs")]
impl std::ops::Deref for crate::UnityEngine::Rendering::LODGroupDataPool_LodGroupShaderIDs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LODGroupDataPool+LodGroupShaderIDs")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::LODGroupDataPool_LodGroupShaderIDs {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LODGroupDataPool+LodGroupShaderIDs")]
impl crate::UnityEngine::Rendering::LODGroupDataPool_LodGroupShaderIDs {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LODGroupDataPool+LodGroupShaderIDs")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::LODGroupDataPool_LodGroupShaderIDs
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
