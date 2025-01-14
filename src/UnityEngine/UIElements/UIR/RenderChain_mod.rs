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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::RenderChain {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "RenderChain";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData,
                1usize,
            >("AccessRenderNodeData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AccessRenderNodeData", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocCommand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::RenderChainCommand,
                >,
                0usize,
            >("AllocCommand")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AllocCommand", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn AppendTexture(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        id: crate::UnityEngine::UIElements::TextureId,
        isAtlas: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                    crate::UnityEngine::UIElements::TextureId,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("AppendTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AppendTexture", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve, src, id, isAtlas))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChildWillBeRemoved(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ChildWillBeRemoved")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ChildWillBeRemoved", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::BaseVisualElementPanel,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::UIR::UIRenderDevice,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::UIR::VectorImageManager,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Constructor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Constructor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (panelObj, deviceObj, atlas, vectorImageMan))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Destructor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Destructor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Destructor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (disposing))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawStats(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("DrawStats")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DrawStats", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureFitsDepth(
        &mut self,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("EnsureFitsDepth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnsureFitsDepth", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (depth))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeCommand(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::RenderChainCommand,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FreeCommand")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FreeCommand", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cmd))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstElementInPanel(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                1usize,
            >("GetFirstElementInPanel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFirstElementInPanel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = unsafe { method.invoke_unchecked((), (ve)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStandardMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                0usize,
            >("GetStandardMaterial")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStandardMaterial", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStandardWorldSpaceMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                0usize,
            >("GetStandardWorldSpaceMaterial")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStandardWorldSpaceMaterial", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::BaseRuntimePanel,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::UIR::RenderChain,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("OnRegisterIntermediateRendererMat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnRegisterIntermediateRendererMat", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (rtp, renderChain, rnd, camera, sameDistanceSortPriority),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnRegisterIntermediateRenderers(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnRegisterIntermediateRenderers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnRegisterIntermediateRenderers", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (camera))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnRenderCommandAdded(
        &mut self,
        command: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::RenderChainCommand,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnRenderCommandAdded")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnRenderCommandAdded", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (command))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnRenderCommandsRemoved")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnRenderCommandsRemoved", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (firstCommand, lastCommand))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnRenderNodeExecute(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnRenderNodeExecute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnRenderNodeExecute", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ProcessChanges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessChanges", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Render(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Render")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Render", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RepaintTexturedElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RepaintTexturedElements")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RepaintTexturedElements", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetTextures(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ResetTextures")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ResetTextures", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnChildAdded(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UIEOnChildAdded")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UIEOnChildAdded", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnChildRemoving(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UIEOnChildRemoving")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UIEOnChildRemoving", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnChildrenReordered(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UIEOnChildrenReordered")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UIEOnChildrenReordered", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnClippingChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        hierarchical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UIEOnClippingChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UIEOnClippingChanged", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve, hierarchical))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnColorChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UIEOnColorChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UIEOnColorChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnOpacityChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        hierarchical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UIEOnOpacityChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UIEOnOpacityChanged", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve, hierarchical))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnOpacityIdChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UIEOnOpacityIdChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UIEOnOpacityIdChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnRenderHintsChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UIEOnRenderHintsChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UIEOnRenderHintsChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnTransformOrSizeChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        transformChanged: bool,
        clipRectSizeChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("UIEOnTransformOrSizeChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UIEOnTransformOrSizeChanged", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve, transformChanged, clipRectSizeChanged))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UIEOnVisualsChanged(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        hierarchical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UIEOnVisualsChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UIEOnVisualsChanged", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve, hierarchical))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::BaseVisualElementPanel,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (panel))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_atlas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>,
                0usize,
            >("get_atlas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_atlas", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::AtlasBase,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_device(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::UIRenderDevice>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::UIRenderDevice,
                >,
                0usize,
            >("get_device")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_device", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_disposed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_disposed", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_drawInCameras(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_drawInCameras")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_drawInCameras", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_drawStats(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_drawStats")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_drawStats", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_indicesPool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::TempAllocator_1<u16>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::TempAllocator_1<u16>,
                >,
                0usize,
            >("get_indicesPool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_indicesPool", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::TempAllocator_1<u16>,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_jobManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::JobManager>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::JobManager,
                >,
                0usize,
            >("get_jobManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_jobManager", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::JobManager,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_opacityIdAccelerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator,
                >,
                0usize,
            >("get_opacityIdAccelerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_opacityIdAccelerator", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_painter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter,
                >,
                0usize,
            >("get_painter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_painter", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_panel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseVisualElementPanel>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::BaseVisualElementPanel,
                >,
                0usize,
            >("get_panel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_panel", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_vectorImageManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::VectorImageManager,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::VectorImageManager,
                >,
                0usize,
            >("get_vectorImageManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_vectorImageManager", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::VectorImageManager,
        > = unsafe { method.invoke_unchecked(self, ()) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::TempAllocator_1<
                        crate::UnityEngine::UIElements::Vertex,
                    >,
                >,
                0usize,
            >("get_vertsPool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_vertsPool", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::TempAllocator_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_atlas(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_atlas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_atlas", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultShader(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_defaultShader")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_defaultShader", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultWorldSpaceShader(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_defaultWorldSpaceShader")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_defaultWorldSpaceShader", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_device(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::UIRenderDevice,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_device")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_device", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_disposed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_disposed", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_drawInCameras(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_drawInCameras")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_drawInCameras", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_drawStats(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_drawStats")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_drawStats", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_indicesPool(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::TempAllocator_1<u16>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::TempAllocator_1<u16>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_indicesPool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_indicesPool", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_jobManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::JobManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::JobManager,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_jobManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_jobManager", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_opacityIdAccelerator(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_opacityIdAccelerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_opacityIdAccelerator", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_painter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_painter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_painter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_panel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::BaseVisualElementPanel,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_panel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_panel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_vectorImageManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::VectorImageManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::VectorImageManager,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_vectorImageManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_vectorImageManager", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::TempAllocator_1<
                        crate::UnityEngine::UIElements::Vertex,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_vertsPool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_vertsPool", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::RenderChain_DepthOrderedDirtyTracking {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "RenderChain/DepthOrderedDirtyTracking";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+DepthOrderedDirtyTracking")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::RenderChain_DepthOrderedDirtyTracking {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+DepthOrderedDirtyTracking")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::RenderChain_DepthOrderedDirtyTracking {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+DepthOrderedDirtyTracking")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::RenderChain_DepthOrderedDirtyTracking {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+DepthOrderedDirtyTracking")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::RenderChain_DepthOrderedDirtyTracking {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    crate::UnityEngine::UIElements::UIR::RenderDataDirtyTypes,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ClearDirty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ClearDirty", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve, dirtyTypesInverse))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureFits(
        &mut self,
        maxDepth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("EnsureFits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnsureFits", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (maxDepth))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDirty(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyTypes: crate::UnityEngine::UIElements::UIR::RenderDataDirtyTypes,
        dirtyTypeClass: crate::UnityEngine::UIElements::UIR::RenderDataDirtyTypeClasses,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    crate::UnityEngine::UIElements::UIR::RenderDataDirtyTypes,
                    crate::UnityEngine::UIElements::UIR::RenderDataDirtyTypeClasses,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("RegisterDirty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterDirty", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve, dirtyTypes, dirtyTypeClass))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+RenderChain+RenderChainStaticIndexAllocator"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderChain_RenderChainStaticIndexAllocator {}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+RenderChain+RenderChainStaticIndexAllocator"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderChainStaticIndexAllocator {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "RenderChain/RenderChainStaticIndexAllocator";
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
#[cfg(
    feature = "UnityEngine+UIElements+UIR+RenderChain+RenderChainStaticIndexAllocator"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderChainStaticIndexAllocator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+RenderChain+RenderChainStaticIndexAllocator"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderChainStaticIndexAllocator {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+RenderChain+RenderChainStaticIndexAllocator"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderChainStaticIndexAllocator {
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
#[cfg(
    feature = "UnityEngine+UIElements+UIR+RenderChain+RenderChainStaticIndexAllocator"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderChainStaticIndexAllocator {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::RenderChain,
                >,
                1usize,
            >("AccessIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AccessIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        > = unsafe { method.invoke_unchecked((), (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn AllocateIndex(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::RenderChain,
                >),
                i32,
                1usize,
            >("AllocateIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AllocateIndex", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (renderChain)) };
        Ok(__cordl_ret.into())
    }
    pub fn FreeIndex(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FreeIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FreeIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (index))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+RenderNodeData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "RenderChain/RenderNodeData";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+RenderNodeData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+RenderNodeData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+RenderNodeData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChain+RenderNodeData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::RenderChain_RenderNodeData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
