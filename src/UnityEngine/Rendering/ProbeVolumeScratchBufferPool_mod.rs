#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeScratchBufferPool")]
#[repr(C)]
#[derive(Debug)]
pub struct ProbeVolumeScratchBufferPool {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _chunkSize_k__BackingField: i32,
    pub _maxChunkCount_k__BackingField: i32,
    pub m_L0Size: i32,
    pub m_L1Size: i32,
    pub m_ValiditySize: i32,
    pub m_ValidityLayerCount: i32,
    pub m_L2Size: i32,
    pub m_ProbeOcclusionSize: i32,
    pub m_SkyOcclusionSize: i32,
    pub m_SkyShadingDirectionSize: i32,
    pub m_CurrentlyAllocatedChunkCount: i32,
    pub m_Pools: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool,
            >,
        >,
    >,
    pub m_Layouts: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBufferLayout,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeScratchBufferPool")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeVolumeScratchBufferPool";
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
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeScratchBufferPool")]
impl std::ops::Deref for crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeScratchBufferPool")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeScratchBufferPool")]
impl crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool {
    #[cfg(feature = "UnityEngine+Rendering+ProbeVolumeScratchBufferPool+ScratchBufferPool")]
    pub type ScratchBufferPool =
        crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool;
    pub fn AllocateScratchBuffer(
        &mut self,
        chunkCount: i32,
        scratchBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBuffer,
            >,
        >,
        layout: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBufferLayout,
        >,
        allocateGraphicsBuffers: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBuffer,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBufferLayout,
                            >,
                            bool,
                        ),
                        bool,
                        4usize,
                    >("AllocateScratchBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocateScratchBuffer", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (chunkCount, scratchBuffer, layout, allocateGraphicsBuffers),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Cleanup(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Cleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Cleanup",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateScratchBuffer(
        &mut self,
        chunkCount: i32,
        allocateGraphicsBuffers: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBuffer,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, bool),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBuffer,
                        >,
                        2usize,
                    >("CreateScratchBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateScratchBuffer", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBuffer,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (chunkCount, allocateGraphicsBuffers))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateScratchBufferLayout(
        &mut self,
        chunkCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBufferLayout,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBufferLayout,
                        1usize,
                    >("GetOrCreateScratchBufferLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOrCreateScratchBufferLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBufferLayout = unsafe {
            cordl_method_info.invoke_unchecked(self, (chunkCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bakingSet: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProbeVolumeBakingSet>,
        shBands: crate::UnityEngine::Rendering::ProbeVolumeSHBands,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bakingSet, shBands))?;
        Ok(__cordl_object.into())
    }
    pub fn ReleaseScratchBuffer(
        &mut self,
        scratchBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReleaseScratchBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseScratchBuffer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (scratchBuffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bakingSet: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProbeVolumeBakingSet>,
        shBands: crate::UnityEngine::Rendering::ProbeVolumeSHBands,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ProbeVolumeBakingSet,
                        >,
                        crate::UnityEngine::Rendering::ProbeVolumeSHBands,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (bakingSet, shBands))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_allocatedMemory(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_allocatedMemory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_allocatedMemory",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_chunkSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_chunkSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_chunkSize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxChunkCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_maxChunkCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_maxChunkCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_chunkSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_chunkSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_chunkSize",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_maxChunkCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_maxChunkCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_maxChunkCount",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeScratchBufferPool")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeScratchBufferPool+ScratchBufferPool")]
#[repr(C)]
#[derive(Debug)]
pub struct ProbeVolumeScratchBufferPool_ScratchBufferPool {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub chunkCount: i32,
    pub pool: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBuffer,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeScratchBufferPool+ScratchBufferPool")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeVolumeScratchBufferPool/ScratchBufferPool";
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
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeScratchBufferPool+ScratchBufferPool")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeScratchBufferPool+ScratchBufferPool")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeScratchBufferPool+ScratchBufferPool")]
impl crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool {
    pub fn CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool,
                        >),
                        i32,
                        1usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_0(
        chunkCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (chunkCount))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_1(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        chunkCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (chunkCount))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeScratchBufferPool+ScratchBufferPool")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeScratchBufferPool+ScratchBufferPool")]
impl
    AsRef<
        crate::System::IComparable_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool,
            >,
        >,
    > for crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeScratchBufferPool+ScratchBufferPool")]
impl
    AsMut<
        crate::System::IComparable_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool,
            >,
        >,
    > for crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ProbeVolumeScratchBufferPool_ScratchBufferPool,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
