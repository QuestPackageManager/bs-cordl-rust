#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicsSettings {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::GraphicsSettings =>
    "UnityEngine.Rendering"."GraphicsSettings"
);
#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
impl std::ops::Deref for crate::UnityEngine::Rendering::GraphicsSettings {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::GraphicsSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
impl crate::UnityEngine::Rendering::GraphicsSettings {
    pub fn get_INTERNAL_currentRenderPipeline() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ScriptableObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_INTERNAL_currentRenderPipeline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_INTERNAL_defaultRenderPipeline() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ScriptableObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_INTERNAL_defaultRenderPipeline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentRenderPipeline() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RenderPipelineAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderPipelineAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_currentRenderPipeline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultRenderPipeline() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RenderPipelineAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderPipelineAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultRenderPipeline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightsUseLinearIntensity() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_lightsUseLinearIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderPipelineAsset() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RenderPipelineAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderPipelineAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_renderPipelineAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_INTERNAL_defaultRenderPipeline(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_INTERNAL_defaultRenderPipeline", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultRenderPipeline(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderPipelineAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_defaultRenderPipeline", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_renderPipelineAsset(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderPipelineAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_renderPipelineAsset", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::GraphicsSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
