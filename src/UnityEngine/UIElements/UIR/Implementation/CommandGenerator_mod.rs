#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+CommandGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+CommandGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::Implementation::CommandGenerator =>
    "UnityEngine.UIElements.UIR.Implementation"."CommandGenerator"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+CommandGenerator")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::UIR::Implementation::CommandGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+CommandGenerator")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::Implementation::CommandGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+CommandGenerator")]
impl crate::UnityEngine::UIElements::UIR::Implementation::CommandGenerator {
    pub fn ClosePaintElement(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        closingInfo: crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo,
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClosePaintElement", (ve, closingInfo, renderChain, stats))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeTransformMatrix(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        ancestor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        result: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeTransformMatrix", (ve, ancestor, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateBlitShader(
        colorConversion: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateBlitShader", (colorConversion))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoUpdateOpacityId(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoUpdateOpacityId", (ve, renderChain, mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindClosingCommandInsertionPoint(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prev: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
        next: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindClosingCommandInsertionPoint", (ve, prev, next))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindCommandInsertionPoint(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prev: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
        next: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindCommandInsertionPoint", (ve, prev, next))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlitMaterial(
        mode: crate::UnityEngine::UIElements::VisualElement_RenderTargetMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBlitMaterial", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVerticesTransformInfo(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        transform: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVerticesTransformInfo", (ve, transform))?;
        Ok(__cordl_ret.into())
    }
    pub fn InjectClosingCommandInBetween(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
        prev: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
        next: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InjectClosingCommandInBetween", (renderChain, cmd, prev, next))?;
        Ok(__cordl_ret.into())
    }
    pub fn InjectClosingMeshDrawCommand(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        cmdPrev: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
        cmdNext: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        indexCount: i32,
        indexOffset: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        texture: crate::UnityEngine::UIElements::TextureId,
        stencilRef: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InjectClosingMeshDrawCommand",
                (
                    renderChain,
                    ve,
                    cmdPrev,
                    cmdNext,
                    mesh,
                    indexCount,
                    indexOffset,
                    material,
                    texture,
                    stencilRef,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InjectCommandInBetween(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
        prev: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
        next: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InjectCommandInBetween", (renderChain, cmd, prev, next))?;
        Ok(__cordl_ret.into())
    }
    pub fn InjectMeshDrawCommand(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        cmdPrev: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
        cmdNext: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        indexCount: i32,
        indexOffset: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        texture: crate::UnityEngine::UIElements::TextureId,
        stencilRef: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InjectMeshDrawCommand",
                (
                    renderChain,
                    ve,
                    cmdPrev,
                    cmdNext,
                    mesh,
                    indexCount,
                    indexOffset,
                    material,
                    texture,
                    stencilRef,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeGenerateVisualContent(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        ctx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshGenerationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeGenerateVisualContent", (ve, ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsParentOrAncestorOf(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsParentOrAncestorOf", (ve, child))?;
        Ok(__cordl_ret.into())
    }
    pub fn NudgeVerticesToNewSpace(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        device: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NudgeVerticesToNewSpace", (ve, renderChain, device))?;
        Ok(__cordl_ret.into())
    }
    pub fn PaintElement(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PaintElement", (renderChain, ve, stats))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNudgeVertices(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        device: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
        >,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        src: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        dst: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareNudgeVertices", (ve, device, mesh, src, dst, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetCommands(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetCommands", (renderChain, ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateOpacityId(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateOpacityId", (ve, renderChain))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateOrAllocate(
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        >,
        vertexCount: i32,
        indexCount: i32,
        device: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
        >,
        verts: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        >,
        indices: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<u16>,
        >,
        indexOffset: quest_hook::libil2cpp::ByRefMut<u16>,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "UpdateOrAllocate",
                (
                    data,
                    vertexCount,
                    indexCount,
                    device,
                    verts,
                    indices,
                    indexOffset,
                    stats,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+CommandGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::Implementation::CommandGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
