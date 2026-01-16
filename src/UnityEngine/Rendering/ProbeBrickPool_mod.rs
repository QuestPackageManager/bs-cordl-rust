#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool")]
#[repr(C)]
#[derive(Debug)]
pub struct ProbeBrickPool {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _estimatedVMemCost_k__BackingField: i32,
    pub m_Pool: crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation,
    pub m_NextFreeChunk: crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
    pub m_FreeList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
        >,
    >,
    pub m_AvailableChunkCount: i32,
    pub m_SHBands: crate::UnityEngine::Rendering::ProbeVolumeSHBands,
    pub m_ContainsValidity: bool,
    pub m_ContainsProbeOcclusion: bool,
    pub m_ContainsRenderingLayers: bool,
    pub m_ContainsSkyOcclusion: bool,
    pub m_ContainsSkyShadingDirection: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::ProbeBrickPool {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeBrickPool";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+ProbeBrickPool")]
impl std::ops::Deref for crate::UnityEngine::Rendering::ProbeBrickPool {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeBrickPool")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::ProbeBrickPool {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeBrickPool")]
impl crate::UnityEngine::Rendering::ProbeBrickPool {
    pub const kBrickCellCount: i32 = 3i32;
    pub const kBrickProbeCountPerDim: i32 = 4i32;
    pub const kBrickProbeCountTotal: i32 = 64i32;
    pub const kChunkProbeCountPerDim: i32 = 512i32;
    pub const kChunkSizeInBricks: i32 = 128i32;
    pub const kMaxPoolWidth: i32 = 2048i32;
    #[cfg(feature = "UnityEngine+Rendering+ProbeBrickPool+BrickChunkAlloc")]
    pub type BrickChunkAlloc = crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc;
    #[cfg(feature = "UnityEngine+Rendering+ProbeBrickPool+DataLocation")]
    pub type DataLocation = crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation;
    pub fn Allocate(
        &mut self,
        numberOfBrickChunks: i32,
        outAllocations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
            >,
        >,
        ignoreErrorLog: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
                                >,
                            >,
                            bool,
                        ),
                        bool,
                        3usize,
                    >("Allocate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Allocate", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (numberOfBrickChunks, outAllocations, ignoreErrorLog),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocatePool(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AllocatePool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocatePool", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (width, height, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Cleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Cleanup",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDataLocation(
        numProbes: i32,
        compressed: bool,
        bands: crate::UnityEngine::Rendering::ProbeVolumeSHBands,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        allocateRendertexture: bool,
        allocateValidityData: bool,
        allocateRenderingLayers: bool,
        allocateSkyOcclusionData: bool,
        allocateSkyShadingDirectionData: bool,
        allocateProbeOcclusionData: bool,
        allocatedBytes: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            bool,
                            crate::UnityEngine::Rendering::ProbeVolumeSHBands,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            bool,
                            bool,
                            bool,
                            bool,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation,
                        11usize,
                    >("CreateDataLocation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateDataLocation", 11usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        numProbes,
                        compressed,
                        bands,
                        name,
                        allocateRendertexture,
                        allocateValidityData,
                        allocateRenderingLayers,
                        allocateSkyOcclusionData,
                        allocateSkyShadingDirectionData,
                        allocateProbeOcclusionData,
                        allocatedBytes,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDataTexture(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        allocateRendertexture: bool,
        allocatedBytes: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        7usize,
                    >("CreateDataTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateDataTexture", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        width,
                        height,
                        depth,
                        format,
                        name,
                        allocateRendertexture,
                        allocatedBytes,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Deallocate(
        &mut self,
        allocations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Deallocate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Deallocate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (allocations))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DerivePoolSizeFromBudget(
        memoryBudget: crate::UnityEngine::Rendering::ProbeVolumeTextureMemoryBudget,
        width: quest_hook::libil2cpp::ByRefMut<i32>,
        height: quest_hook::libil2cpp::ByRefMut<i32>,
        depth: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Rendering::ProbeVolumeTextureMemoryBudget,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("DerivePoolSizeFromBudget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DerivePoolSizeFromBudget", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (memoryBudget, width, height, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DivRoundUp(x: i32, y: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32), i32, 2usize>("DivRoundUp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DivRoundUp", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (x, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureTextureValidity_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("EnsureTextureValidity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnsureTextureValidity", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureTextureValidity__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        &mut self,
        renderingLayers: bool,
        skyOcclusion: bool,
        skyDirection: bool,
        probeOcclusion: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool, bool, bool),
                        bool,
                        4usize,
                    >("EnsureTextureValidity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnsureTextureValidity", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (renderingLayers, skyOcclusion, skyDirection, probeOcclusion),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EstimateMemoryCost(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        ),
                        i32,
                        4usize,
                    >("EstimateMemoryCost")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EstimateMemoryCost", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (width, height, depth, format))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EstimateMemoryCostForBlending(
        memoryBudget: crate::UnityEngine::Rendering::ProbeVolumeTextureMemoryBudget,
        compressed: bool,
        bands: crate::UnityEngine::Rendering::ProbeVolumeSHBands,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Rendering::ProbeVolumeTextureMemoryBudget,
                            bool,
                            crate::UnityEngine::Rendering::ProbeVolumeSHBands,
                        ),
                        i32,
                        3usize,
                    >("EstimateMemoryCostForBlending")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EstimateMemoryCostForBlending", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (memoryBudget, compressed, bands))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetChunkCount(brickCount: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i32, 1usize>("GetChunkCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetChunkCount", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (brickCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetChunkSizeInBrickCount() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("GetChunkSizeInBrickCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetChunkSizeInBrickCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetChunkSizeInProbeCount() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("GetChunkSizeInProbeCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetChunkSizeInProbeCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPoolDimensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3Int> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Vector3Int,
                        0usize,
                    >("GetPoolDimensions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPoolDimensions", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3Int = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPoolHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetPoolHeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPoolHeight", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPoolWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetPoolWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPoolWidth", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetProbeOcclusionTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        0usize,
                    >("GetProbeOcclusionTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetProbeOcclusionTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRemainingChunkCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetRemainingChunkCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRemainingChunkCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRuntimeResources(
        &mut self,
        rr: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ProbeReferenceVolume_RuntimeResources,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ProbeReferenceVolume_RuntimeResources,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetRuntimeResources")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRuntimeResources", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSkyOcclusionTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        0usize,
                    >("GetSkyOcclusionTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSkyOcclusionTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSkyShadingDirectionIndicesTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        0usize,
                    >("GetSkyShadingDirectionIndicesTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSkyShadingDirectionIndicesTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetValidityTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        0usize,
                    >("GetValidityTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetValidityTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        memoryBudget: crate::UnityEngine::Rendering::ProbeVolumeTextureMemoryBudget,
        shBands: crate::UnityEngine::Rendering::ProbeVolumeSHBands,
        allocateValidityData: bool,
        allocateRenderingLayerData: bool,
        allocateSkyOcclusion: bool,
        allocateSkyShadingData: bool,
        allocateProbeOcclusionData: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    memoryBudget,
                    shBands,
                    allocateValidityData,
                    allocateRenderingLayerData,
                    allocateSkyOcclusion,
                    allocateSkyShadingData,
                    allocateProbeOcclusionData,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ProbeCountToDataLocSize(
        numProbes: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3Int> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        crate::UnityEngine::Vector3Int,
                        1usize,
                    >("ProbeCountToDataLocSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProbeCountToDataLocSize", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3Int = unsafe {
            cordl_method_info.invoke_unchecked((), (numProbes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValidity(
        &mut self,
        source: crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation,
        srcLocations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
            >,
        >,
        dstLocations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
            >,
        >,
        destStartIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("UpdateValidity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateValidity", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (source, srcLocations, dstLocations, destStartIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update_CommandBuffer_ProbeReferenceVolume_CellStreamingScratchBuffer_ProbeReferenceVolume_CellStreamingScratchBufferLayout_List_1__cordl_bool_Texture_ProbeVolumeSHBands__cordl_bool_Texture__cordl_bool_Texture__cordl_bool1(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        dataBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBuffer,
        >,
        layout: crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBufferLayout,
        dstLocations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
            >,
        >,
        updateSharedData: bool,
        validityTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        bands: crate::UnityEngine::Rendering::ProbeVolumeSHBands,
        skyOcclusion: bool,
        skyOcclusionTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        skyShadingDirections: bool,
        skyShadingDirectionsTexture: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Texture,
        >,
        probeOcclusion: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBuffer,
                            >,
                            crate::UnityEngine::Rendering::ProbeReferenceVolume_CellStreamingScratchBufferLayout,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
                                >,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::UnityEngine::Rendering::ProbeVolumeSHBands,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Update",
                            12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        cmd,
                        dataBuffer,
                        layout,
                        dstLocations,
                        updateSharedData,
                        validityTexture,
                        bands,
                        skyOcclusion,
                        skyOcclusionTexture,
                        skyShadingDirections,
                        skyShadingDirectionsTexture,
                        probeOcclusion,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update_ProbeBrickPool_DataLocation_List_1_List_1_i32_ProbeVolumeSHBands0(
        &mut self,
        source: crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation,
        srcLocations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
            >,
        >,
        dstLocations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
            >,
        >,
        destStartIndex: i32,
        bands: crate::UnityEngine::Rendering::ProbeVolumeSHBands,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc,
                                >,
                            >,
                            i32,
                            crate::UnityEngine::Rendering::ProbeVolumeSHBands,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Update",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (source, srcLocations, dstLocations, destStartIndex, bands),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        memoryBudget: crate::UnityEngine::Rendering::ProbeVolumeTextureMemoryBudget,
        shBands: crate::UnityEngine::Rendering::ProbeVolumeSHBands,
        allocateValidityData: bool,
        allocateRenderingLayerData: bool,
        allocateSkyOcclusion: bool,
        allocateSkyShadingData: bool,
        allocateProbeOcclusionData: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::ProbeVolumeTextureMemoryBudget,
                            crate::UnityEngine::Rendering::ProbeVolumeSHBands,
                            bool,
                            bool,
                            bool,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        memoryBudget,
                        shBands,
                        allocateValidityData,
                        allocateRenderingLayerData,
                        allocateSkyOcclusion,
                        allocateSkyShadingData,
                        allocateProbeOcclusionData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_estimatedVMemCost(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_estimatedVMemCost")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_estimatedVMemCost", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_estimatedVMemCost(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_estimatedVMemCost")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_estimatedVMemCost", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::ProbeBrickPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+BrickChunkAlloc")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProbeBrickPool_BrickChunkAlloc {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+BrickChunkAlloc")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeBrickPool/BrickChunkAlloc";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+BrickChunkAlloc")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+BrickChunkAlloc")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+BrickChunkAlloc")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+BrickChunkAlloc")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+BrickChunkAlloc")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeBrickPool+BrickChunkAlloc")]
impl crate::UnityEngine::Rendering::ProbeBrickPool_BrickChunkAlloc {
    pub fn flattenIndex(
        &mut self,
        sx: i32,
        sy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), i32, 2usize>("flattenIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "flattenIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (sx, sy))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+DataLocation")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProbeBrickPool_DataLocation {
    pub TexL0_L1rx: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub TexL1_G_ry: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub TexL1_B_rz: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub TexL2_0: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub TexL2_1: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub TexL2_2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub TexL2_3: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub TexProbeOcclusion: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub TexValidity: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub TexSkyOcclusion: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub TexSkyShadingDirectionIndices: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Texture,
    >,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+DataLocation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeBrickPool/DataLocation";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+DataLocation")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+DataLocation")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+DataLocation")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+DataLocation")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeBrickPool+DataLocation")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeBrickPool+DataLocation")]
impl crate::UnityEngine::Rendering::ProbeBrickPool_DataLocation {
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Cleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Cleanup",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
