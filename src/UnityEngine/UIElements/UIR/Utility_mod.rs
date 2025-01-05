#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
#[repr(C)]
#[derive(Debug)]
pub struct Utility {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::Utility =>
    "UnityEngine.UIElements.UIR"."Utility"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::Utility {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::Utility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
impl crate::UnityEngine::UIElements::UIR::Utility {
    #[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBufferType")]
    pub type GPUBufferType = crate::UnityEngine::UIElements::UIR::Utility_GPUBufferType;
    #[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
    pub type GPUBuffer_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<
        T,
    >;
    pub fn AllocateBuffer(
        elementCount: i32,
        elementStride: i32,
        vertexBuffer: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocateBuffer", (elementCount, elementStride, vertexBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn CPUFencePassed(fence: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CPUFencePassed", (fence))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateStencilState(
        stencilState: crate::UnityEngine::Rendering::StencilState,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateStencilState", (stencilState))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateStencilState_Injected(
        stencilState: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::StencilState,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateStencilState_Injected", (stencilState))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableScissor() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisableScissor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawRanges(
        ib: crate::System::IntPtr,
        vertexStreams: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        streamCount: i32,
        ranges: crate::System::IntPtr,
        rangeCount: i32,
        vertexDecl: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DrawRanges",
                (ib, vertexStreams, streamCount, ranges, rangeCount, vertexDecl),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeBuffer(
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FreeBuffer", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveViewport() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::RectInt,
    > {
        let __cordl_ret: crate::UnityEngine::RectInt = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActiveViewport", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveViewport_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActiveViewport_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnityProjectionMatrix() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Matrix4x4,
    > {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnityProjectionMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnityProjectionMatrix_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnityProjectionMatrix_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexDeclaration(
        vertexAttributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::VertexAttributeDescriptor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVertexDeclaration", (vertexAttributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasMappedBufferRange() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasMappedBufferRange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertCPUFence() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InsertCPUFence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyOfUIREvents(
        subscribe: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyOfUIREvents", (subscribe))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProfileDrawChainBegin() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProfileDrawChainBegin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProfileDrawChainEnd() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProfileDrawChainEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseEngineUpdate() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaiseEngineUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseFlushPendingResources() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaiseFlushPendingResources", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseGraphicsResourcesRecreate(
        recreate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaiseGraphicsResourcesRecreate", (recreate))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseRegisterIntermediateRenderers(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaiseRegisterIntermediateRenderers", (camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseRenderNodeAdd(
        userData: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaiseRenderNodeAdd", (userData))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseRenderNodeCleanup(
        userData: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaiseRenderNodeCleanup", (userData))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseRenderNodeExecute(
        userData: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaiseRenderNodeExecute", (userData))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterIntermediateRenderer(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        transform: crate::UnityEngine::Matrix4x4,
        aabb: crate::UnityEngine::Bounds,
        renderLayer: i32,
        shadowCasting: i32,
        receiveShadows: bool,
        sameDistanceSortPriority: i32,
        sceneCullingMask: u64,
        rendererCallbackFlags: i32,
        userData: crate::System::IntPtr,
        userDataSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RegisterIntermediateRenderer",
                (
                    camera,
                    material,
                    transform,
                    aabb,
                    renderLayer,
                    shadowCasting,
                    receiveShadows,
                    sameDistanceSortPriority,
                    sceneCullingMask,
                    rendererCallbackFlags,
                    userData,
                    userDataSize,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterIntermediateRenderer_Injected(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        transform: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        aabb: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        renderLayer: i32,
        shadowCasting: i32,
        receiveShadows: bool,
        sameDistanceSortPriority: i32,
        sceneCullingMask: u64,
        rendererCallbackFlags: i32,
        userData: crate::System::IntPtr,
        userDataSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RegisterIntermediateRenderer_Injected",
                (
                    camera,
                    material,
                    transform,
                    aabb,
                    renderLayer,
                    shadowCasting,
                    receiveShadows,
                    sameDistanceSortPriority,
                    sceneCullingMask,
                    rendererCallbackFlags,
                    userData,
                    userDataSize,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPropertyBlock(
        props: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetPropertyBlock", (props))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetScissorRect(
        scissorRect: crate::UnityEngine::RectInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetScissorRect", (scissorRect))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetScissorRect_Injected(
        scissorRect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetScissorRect_Injected", (scissorRect))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStencilState(
        stencilState: crate::System::IntPtr,
        stencilRef: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStencilState", (stencilState, stencilRef))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVectorArray_IntPtr_i32_1(
        props: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        name: i32,
        vector4s: crate::System::IntPtr,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetVectorArray", (props, name, vector4s, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVectorArray_NativeSlice_1_0<T>(
        props: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        name: i32,
        vector4s: crate::Unity::Collections::NativeSlice_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetVectorArray", (props, name, vector4s))?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncRenderThread() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SyncRenderThread", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateBufferRanges(
        buffer: crate::System::IntPtr,
        ranges: crate::System::IntPtr,
        rangeCount: i32,
        writeRangeStart: i32,
        writeRangeEnd: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "UpdateBufferRanges",
                (buffer, ranges, rangeCount, writeRangeStart, writeRangeEnd),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitForCPUFencePassed(
        fence: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitForCPUFencePassed", (fence))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_EngineUpdate(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_EngineUpdate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_FlushPendingResources(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_FlushPendingResources", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_GraphicsResourcesRecreate(
        value: quest_hook::libil2cpp::Gc<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_GraphicsResourcesRecreate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_RegisterIntermediateRenderers(
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_RegisterIntermediateRenderers", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_RenderNodeExecute(
        value: quest_hook::libil2cpp::Gc<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_RenderNodeExecute", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_EngineUpdate(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_EngineUpdate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_FlushPendingResources(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_FlushPendingResources", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_GraphicsResourcesRecreate(
        value: quest_hook::libil2cpp::Gc<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_GraphicsResourcesRecreate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_RegisterIntermediateRenderers(
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_RegisterIntermediateRenderers", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_RenderNodeExecute(
        value: quest_hook::libil2cpp::Gc<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_RenderNodeExecute", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::UIR::Utility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBufferType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Utility_GPUBufferType {
    #[default]
    Index = 1i32,
    Vertex = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBufferType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::Utility_GPUBufferType => "UnityEngine.UIElements.UIR"
    ."Utility/GPUBufferType"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Utility_GPUBuffer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub buffer: crate::System::IntPtr,
    pub elemCount: i32,
    pub elemStride: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1 < T > =>
    "UnityEngine.UIElements.UIR"."Utility/GPUBuffer`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        elementCount: i32,
        _cordl_type: crate::UnityEngine::UIElements::UIR::Utility_GPUBufferType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (elementCount, _cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateRanges(
        &mut self,
        ranges: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::UIR::GfxUpdateBufferRange,
        >,
        rangesMin: i32,
        rangesMax: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRanges", (ranges, rangesMin, rangesMax))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        elementCount: i32,
        _cordl_type: crate::UnityEngine::UIElements::UIR::Utility_GPUBufferType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (elementCount, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BufferPointer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("get_BufferPointer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ElementStride(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ElementStride", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Utility+GPUBuffer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<T> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
