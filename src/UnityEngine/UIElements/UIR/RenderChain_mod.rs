#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderChain {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_FirstCommand: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    >,
    pub m_DirtyTracker: crate::UnityEngine::UIElements::UIR::RenderChain_DepthOrderedDirtyTracking,
    pub m_CommandPool: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::LinkedPool_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::RenderChainCommand,
            >,
        >,
    >,
    pub m_TexturePool: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::BasicNodePool_1<
            crate::UnityEngine::UIElements::UIR::TextureEntry,
        >,
    >,
    pub m_RenderNodesData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData,
        >,
    >,
    pub m_DefaultShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub m_DefaultWorldSpaceShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub m_DefaultMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_DefaultWorldSpaceMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_BlockDirtyRegistration: bool,
    pub m_StaticIndex: i32,
    pub m_ActiveRenderNodes: i32,
    pub m_CustomMaterialCommands: i32,
    pub m_Stats: crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
    pub m_StatsElementsAdded: u32,
    pub m_StatsElementsRemoved: u32,
    pub m_TextureRegistry: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::TextureRegistry,
    >,
    pub _opacityIdAccelerator_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator,
    >,
    pub _disposed_k__BackingField: bool,
    pub _panel_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::BaseVisualElementPanel,
    >,
    pub _device_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::UIRenderDevice,
    >,
    pub _atlas_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::AtlasBase,
    >,
    pub _vectorImageManager_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::VectorImageManager,
    >,
    pub _vertsPool_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::TempAllocator_1<
            crate::UnityEngine::UIElements::Vertex,
        >,
    >,
    pub _indicesPool_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::TempAllocator_1<u16>,
    >,
    pub _jobManager_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::JobManager,
    >,
    pub shaderInfoAllocator: crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator,
    pub _painter_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter,
    >,
    pub _drawStats_k__BackingField: bool,
    pub _drawInCameras_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::RenderChain =>
    "UnityEngine.UIElements.UIR"."RenderChain"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::RenderChain {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::RenderChain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain")]
impl crate::UnityEngine::UIElements::UIR::RenderChain {
    #[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+DepthOrderedDirtyTracking")]
    pub type DepthOrderedDirtyTracking = crate::UnityEngine::UIElements::UIR::RenderChain_DepthOrderedDirtyTracking;
    #[cfg(
        feature = "UnityEngine+UIElements+UIR+RenderChain+RenderChainStaticIndexAllocator"
    )]
    pub type RenderChainStaticIndexAllocator = crate::UnityEngine::UIElements::UIR::RenderChain_RenderChainStaticIndexAllocator;
    #[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+RenderNodeData")]
    pub type RenderNodeData = crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData;
    pub fn AccessRenderNodeData(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AccessRenderNodeData", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocCommand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        > = __cordl_object.invoke("AllocCommand", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendTexture(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        id: crate::UnityEngine::UIElements::TextureId,
        isAtlas: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendTexture", (ve, src, id, isAtlas))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChildWillBeRemoved(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChildWillBeRemoved", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn Constructor(
        &mut self,
        panelObj: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
        deviceObj: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
        >,
        atlas: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>,
        vectorImageMan: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::VectorImageManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Constructor", (panelObj, deviceObj, atlas, vectorImageMan))?;
        Ok(__cordl_ret.into())
    }
    pub fn Destructor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destructor", ())?;
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
    pub fn DrawStats(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawStats", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureFitsDepth(
        &mut self,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureFitsDepth", (depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeCommand(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeCommand", (cmd))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstElementInPanel(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFirstElementInPanel", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStandardMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("GetStandardMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStandardWorldSpaceMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("GetStandardWorldSpaceMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (panel))?;
        Ok(__cordl_object.into())
    }
    pub fn OnRegisterIntermediateRendererMat(
        rtp: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseRuntimePanel>,
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        rnd: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData,
        >,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        sameDistanceSortPriority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OnRegisterIntermediateRendererMat",
                (rtp, renderChain, rnd, camera, sameDistanceSortPriority),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRegisterIntermediateRenderers(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnRegisterIntermediateRenderers", (camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRenderCommandAdded(
        &mut self,
        command: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRenderCommandAdded", (command))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRenderCommandsRemoved(
        &mut self,
        firstCommand: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
        lastCommand: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRenderCommandsRemoved", (firstCommand, lastCommand))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRenderNodeExecute(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnRenderNodeExecute", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessChanges", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Render(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Render", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RepaintTexturedElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RepaintTexturedElements", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetTextures(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetTextures", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnChildAdded(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UIEOnChildAdded", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnChildRemoving(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UIEOnChildRemoving", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnChildrenReordered(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UIEOnChildrenReordered", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnClippingChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        hierarchical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UIEOnClippingChanged", (ve, hierarchical))?;
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnColorChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UIEOnColorChanged", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnOpacityChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        hierarchical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UIEOnOpacityChanged", (ve, hierarchical))?;
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnOpacityIdChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UIEOnOpacityIdChanged", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnRenderHintsChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UIEOnRenderHintsChanged", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnTransformOrSizeChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        transformChanged: bool,
        clipRectSizeChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UIEOnTransformOrSizeChanged",
                (ve, transformChanged, clipRectSizeChanged),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnVisualsChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        hierarchical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UIEOnVisualsChanged", (ve, hierarchical))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_atlas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::AtlasBase,
        > = __cordl_object.invoke("get_atlas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_device(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::UIRenderDevice>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
        > = __cordl_object.invoke("get_device", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_drawInCameras(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_drawInCameras", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_drawStats(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_drawStats", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_indicesPool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::TempAllocator_1<u16>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::TempAllocator_1<u16>,
        > = __cordl_object.invoke("get_indicesPool", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_jobManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::JobManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::JobManager,
        > = __cordl_object.invoke("get_jobManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_opacityIdAccelerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator,
        > = __cordl_object.invoke("get_opacityIdAccelerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_painter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter,
        > = __cordl_object.invoke("get_painter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_panel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseVisualElementPanel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        > = __cordl_object.invoke("get_panel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vectorImageManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::VectorImageManager,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::VectorImageManager,
        > = __cordl_object.invoke("get_vectorImageManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertsPool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::TempAllocator_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::TempAllocator_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        > = __cordl_object.invoke("get_vertsPool", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_atlas(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atlas", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultShader(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultShader", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultWorldSpaceShader(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultWorldSpaceShader", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_device(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_device", (value))?;
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
    pub fn set_drawInCameras(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_drawInCameras", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_drawStats(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_drawStats", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_indicesPool(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::TempAllocator_1<u16>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_indicesPool", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_jobManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::JobManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_jobManager", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_opacityIdAccelerator(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_opacityIdAccelerator", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_painter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_painter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_panel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_panel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vectorImageManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::VectorImageManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_vectorImageManager", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vertsPool(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::TempAllocator_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_vertsPool", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::RenderChain {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::RenderChain {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::RenderChain {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+DepthOrderedDirtyTracking")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RenderChain_DepthOrderedDirtyTracking {
    pub heads: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        >,
    >,
    pub tails: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        >,
    >,
    pub minDepths: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub maxDepths: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub dirtyID: u32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+DepthOrderedDirtyTracking")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::RenderChain_DepthOrderedDirtyTracking =>
    "UnityEngine.UIElements.UIR"."RenderChain/DepthOrderedDirtyTracking"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+DepthOrderedDirtyTracking")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::RenderChain_DepthOrderedDirtyTracking {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+DepthOrderedDirtyTracking")]
impl crate::UnityEngine::UIElements::UIR::RenderChain_DepthOrderedDirtyTracking {
    pub fn ClearDirty(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyTypesInverse: crate::UnityEngine::UIElements::UIR::RenderDataDirtyTypes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ClearDirty",
            (ve, dirtyTypesInverse),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureFits(
        &mut self,
        maxDepth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "EnsureFits",
            (maxDepth),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDirty(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyTypes: crate::UnityEngine::UIElements::UIR::RenderDataDirtyTypes,
        dirtyTypeClass: crate::UnityEngine::UIElements::UIR::RenderDataDirtyTypeClasses,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RegisterDirty",
            (ve, dirtyTypes, dirtyTypeClass),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+RenderChain+RenderChainStaticIndexAllocator"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RenderChain_RenderChainStaticIndexAllocator {}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+RenderChain+RenderChainStaticIndexAllocator"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::RenderChain_RenderChainStaticIndexAllocator =>
    "UnityEngine.UIElements.UIR"."RenderChain/RenderChainStaticIndexAllocator"
);
#[cfg(
    feature = "UnityEngine+UIElements+UIR+RenderChain+RenderChainStaticIndexAllocator"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderChainStaticIndexAllocator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+RenderChain+RenderChainStaticIndexAllocator"
)]
impl crate::UnityEngine::UIElements::UIR::RenderChain_RenderChainStaticIndexAllocator {
    pub fn AccessIndex(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AccessIndex", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocateIndex(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocateIndex", (renderChain))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeIndex(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FreeIndex", (index))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+RenderNodeData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RenderChain_RenderNodeData {
    pub standardMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub initialMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub matPropBlock: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::MaterialPropertyBlock,
    >,
    pub firstCommand: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    >,
    pub device: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::UIRenderDevice,
    >,
    pub vectorAtlas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub shaderInfoAtlas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub dpiScale: f32,
    pub transformConstants: crate::Unity::Collections::NativeSlice_1<
        crate::UnityEngine::UIElements::UIR::Transform3x4,
    >,
    pub clipRectConstants: crate::Unity::Collections::NativeSlice_1<
        crate::UnityEngine::Vector4,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+RenderNodeData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData =>
    "UnityEngine.UIElements.UIR"."RenderChain/RenderNodeData"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+RenderNodeData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+RenderNodeData")]
impl crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData {}
