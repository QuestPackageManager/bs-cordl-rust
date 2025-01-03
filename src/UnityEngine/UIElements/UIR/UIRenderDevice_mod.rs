#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
#[repr(C)]
#[derive(Debug)]
pub struct UIRenderDevice {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_MockDevice: bool,
    pub m_DefaultStencilState: crate::System::IntPtr,
    pub m_VertexDecl: crate::System::IntPtr,
    pub m_FirstPage: *mut crate::UnityEngine::UIElements::UIR::Page,
    pub m_NextPageVertexCount: u32,
    pub m_LargeMeshVertexCount: u32,
    pub m_IndexToVertexCountRatio: f32,
    pub m_DeferredFrees: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree,
        >,
    >,
    pub m_Updates: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate,
        >,
    >,
    pub m_Fences: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub m_StandardMatProps: *mut crate::UnityEngine::MaterialPropertyBlock,
    pub m_FrameIndex: u32,
    pub m_NextUpdateID: u32,
    pub m_DrawStats: crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics,
    pub m_MeshHandles: *mut crate::UnityEngine::UIElements::UIR::LinkedPool_1<
        *mut crate::UnityEngine::UIElements::UIR::MeshHandle,
    >,
    pub m_DrawParams: *mut crate::UnityEngine::UIElements::UIR::DrawParams,
    pub m_TextureSlotManager: *mut crate::UnityEngine::UIElements::UIR::TextureSlotManager,
    pub _maxVerticesPerPage_k__BackingField: u32,
    pub _breakBatches_k__BackingField: bool,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::UIRenderDevice =>
    "UnityEngine.UIElements.UIR"."UIRenderDevice"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    #[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
    pub type AllocToFree = crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree;
    #[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
    pub type AllocToUpdate = crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate;
    #[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
    pub type DeviceToFree = crate::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree;
    #[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
    pub type DrawStatistics = crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics;
    #[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
    pub type EvaluationState = crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState;
    #[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+__c")]
    pub type __c = crate::UnityEngine::UIElements::UIR::UIRenderDevice___c;
    pub fn ActiveUpdatesForMeshHandle(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate,
            >,
        > = __cordl_object.invoke("ActiveUpdatesForMeshHandle", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdvanceFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AdvanceFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Allocate_MeshHandle_u32__cordl_bool1(
        &mut self,
        meshHandle: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::MeshHandle,
        >,
        vertexCount: u32,
        indexCount: u32,
        vertexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        >,
        indexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<u16>,
        >,
        shortLived: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Allocate",
                (meshHandle, vertexCount, indexCount, vertexData, indexData, shortLived),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Allocate_u32_ByRefMut0(
        &mut self,
        vertexCount: u32,
        indexCount: u32,
        vertexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        >,
        indexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<u16>,
        >,
        indexOffset: quest_hook::libil2cpp::ByRefMut<u16>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::MeshHandle,
        > = __cordl_object
            .invoke(
                "Allocate",
                (vertexCount, indexCount, vertexData, indexData, indexOffset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBatchState(
        &mut self,
        st: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState,
        >,
        allowMaterialChange: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyBatchState", (st, allowMaterialChange))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyDrawCommandState(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
        textureSlot: i32,
        newMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        newMatDiffers: bool,
        st: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ApplyDrawCommandState",
                (cmd, textureSlot, newMat, newMatDiffers, st),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteCreation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteCreation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawRanges<I, T>(
        &mut self,
        ib: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<I>,
        >,
        vb: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T>,
        >,
        ranges: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::UIR::DrawBufferRange,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        I: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawRanges", (ib, vb, ranges))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateChain(
        &mut self,
        head: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
        initialMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        defaultMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        gradientSettings: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        shaderInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        pixelsPerPoint: f32,
        transforms: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::UIR::Transform3x4,
        >,
        clipRects: crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::Vector4>,
        stateMatProps: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::MaterialPropertyBlock,
        >,
        allowMaterialChange: bool,
        immediateException: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Exception,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "EvaluateChain",
                (
                    head,
                    initialMat,
                    defaultMat,
                    gradientSettings,
                    shaderInfo,
                    pixelsPerPoint,
                    transforms,
                    clipRects,
                    stateMatProps,
                    allowMaterialChange,
                    immediateException,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushAllPendingDeviceDisposes() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FlushAllPendingDeviceDisposes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Free(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Free", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GatherDrawStatistics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics = __cordl_object
            .invoke("GatherDrawStatistics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitVertexDeclaration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitVertexDeclaration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn KickRanges(
        &mut self,
        ranges: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        rangesReady: quest_hook::libil2cpp::ByRefMut<i32>,
        rangesStart: quest_hook::libil2cpp::ByRefMut<i32>,
        rangesCount: i32,
        curPage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "KickRanges",
                (ranges, rangesReady, rangesStart, rangesCount, curPage),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New__cordl_bool1(
        initialVertexCapacity: u32,
        initialIndexCapacity: u32,
        mockDevice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (initialVertexCapacity, initialIndexCapacity, mockDevice),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_u32_0(
        initialVertexCapacity: u32,
        initialIndexCapacity: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialVertexCapacity, initialIndexCapacity))?;
        Ok(__cordl_object.into())
    }
    pub fn OnEngineUpdateGlobal() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnEngineUpdateGlobal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnFlushPendingResources() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnFlushPendingResources", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnFrameRenderingBegin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFrameRenderingBegin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareForGfxDeviceRecreate() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareForGfxDeviceRecreate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessDeviceFreeQueue() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessDeviceFreeQueue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PruneUnusedPages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PruneUnusedPages", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PtrToSlice<T>(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeSlice_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PtrToSlice", (p, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryAllocFromPage(
        &mut self,
        page: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
        vertexCount: u32,
        indexCount: u32,
        va: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::UIR::Alloc>,
        ia: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::UIR::Alloc>,
        shortLived: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryAllocFromPage",
                (page, vertexCount, indexCount, va, ia, shortLived),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAfterGPUUsedData(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        vertexCount: u32,
        indexCount: u32,
        vertexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        >,
        indexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<u16>,
        >,
        indexOffset: quest_hook::libil2cpp::ByRefMut<u16>,
        allocToUpdate: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate,
        >,
        copyBackIndices: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UpdateAfterGPUUsedData",
                (
                    mesh,
                    vertexCount,
                    indexCount,
                    vertexData,
                    indexData,
                    indexOffset,
                    allocToUpdate,
                    copyBackIndices,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCopyBackIndices(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        copyBackIndices: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCopyBackIndices", (mesh, copyBackIndices))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateFenceValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFenceValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update_ByRefMut0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        vertexCount: u32,
        vertexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (mesh, vertexCount, vertexData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update_u32_ByRefMut_ByRefMut_ByRefMut1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        vertexCount: u32,
        indexCount: u32,
        vertexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        >,
        indexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<u16>,
        >,
        indexOffset: quest_hook::libil2cpp::ByRefMut<u16>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Update",
                (mesh, vertexCount, indexCount, vertexData, indexData, indexOffset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitOnCpuFence(
        &mut self,
        fence: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WaitOnCpuFence", (fence))?;
        Ok(__cordl_ret.into())
    }
    pub fn WrapUpGfxDeviceRecreate() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WrapUpGfxDeviceRecreate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        initialVertexCapacity: u32,
        initialIndexCapacity: u32,
        mockDevice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialVertexCapacity, initialIndexCapacity, mockDevice))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_u32_0(
        &mut self,
        initialVertexCapacity: u32,
        initialIndexCapacity: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialVertexCapacity, initialIndexCapacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_breakBatches(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_breakBatches", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultShaderInfoTexARGB8() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultShaderInfoTexARGB8", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultShaderInfoTexFloat() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultShaderInfoTexFloat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fullyCreated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_fullyCreated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxVerticesPerPage(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_maxVerticesPerPage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shaderModelIs35() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_shaderModelIs35", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertexTexturingIsAvailable() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_vertexTexturingIsAvailable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_breakBatches(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_breakBatches", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposed", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UIRenderDevice_AllocToFree {
    pub alloc: crate::UnityEngine::UIElements::UIR::Alloc,
    pub page: *mut crate::UnityEngine::UIElements::UIR::Page,
    pub vertices: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree =>
    "UnityEngine.UIElements.UIR"."UIRenderDevice/AllocToFree"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree {}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UIRenderDevice_AllocToUpdate {
    pub id: u32,
    pub allocTime: u32,
    pub meshHandle: *mut crate::UnityEngine::UIElements::UIR::MeshHandle,
    pub permAllocVerts: crate::UnityEngine::UIElements::UIR::Alloc,
    pub permAllocIndices: crate::UnityEngine::UIElements::UIR::Alloc,
    pub permPage: *mut crate::UnityEngine::UIElements::UIR::Page,
    pub copyBackIndices: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate =>
    "UnityEngine.UIElements.UIR"."UIRenderDevice/AllocToUpdate"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate {}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UIRenderDevice_DeviceToFree {
    pub handle: u32,
    pub page: *mut crate::UnityEngine::UIElements::UIR::Page,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree =>
    "UnityEngine.UIElements.UIR"."UIRenderDevice/DeviceToFree"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UIRenderDevice_DrawStatistics {
    pub currentFrameIndex: i32,
    pub totalIndices: u32,
    pub commandCount: u32,
    pub drawCommandCount: u32,
    pub materialSetCount: u32,
    pub drawRangeCount: u32,
    pub drawRangeCallCount: u32,
    pub immediateDraws: u32,
    pub stencilRefChanges: u32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics =>
    "UnityEngine.UIElements.UIR"."UIRenderDevice/DrawStatistics"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics {}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UIRenderDevice_EvaluationState {
    pub stateMatProps: *mut crate::UnityEngine::MaterialPropertyBlock,
    pub defaultMat: *mut crate::UnityEngine::Material,
    pub curState: crate::UnityEngine::UIElements::UIR::State,
    pub curPage: *mut crate::UnityEngine::UIElements::UIR::Page,
    pub mustApplyMaterial: bool,
    pub mustApplyCommonBlock: bool,
    pub mustApplyStateBlock: bool,
    pub mustApplyStencil: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState =>
    "UnityEngine.UIElements.UIR"."UIRenderDevice/EvaluationState"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState {}
