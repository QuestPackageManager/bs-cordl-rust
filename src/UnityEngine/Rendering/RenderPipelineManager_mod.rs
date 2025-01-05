#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderPipelineManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::RenderPipelineManager =>
    "UnityEngine.Rendering"."RenderPipelineManager"
);
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
impl std::ops::Deref for crate::UnityEngine::Rendering::RenderPipelineManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::RenderPipelineManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
impl crate::UnityEngine::Rendering::RenderPipelineManager {
    pub fn CleanupRenderPipeline() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CleanupRenderPipeline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoRenderLoop_Internal(
        pipe: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderPipelineAsset,
        >,
        loopPtr: crate::System::IntPtr,
        renderRequest: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoRenderLoop_Internal", (pipe, loopPtr, renderRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentPipelineAssetType() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentPipelineAssetType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleRenderPipelineChange(
        pipelineAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderPipelineAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HandleRenderPipelineChange", (pipelineAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPipelineRequireCreation() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPipelineRequireCreation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnActiveRenderPipelineAssetChanged(
        from: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
        to: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnActiveRenderPipelineAssetChanged", (from, to))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnActiveRenderPipelineTypeChanged() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnActiveRenderPipelineTypeChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareRenderPipeline(
        pipelineAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderPipelineAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareRenderPipeline", (pipelineAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_beginCameraRendering(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_beginCameraRendering", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentPipeline() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RenderPipeline>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderPipeline,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_currentPipeline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_beginCameraRendering(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_beginCameraRendering", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_currentPipeline(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RenderPipeline>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_currentPipeline", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderPipelineManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
