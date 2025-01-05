#[cfg(feature = "UnityEngine+QualitySettings")]
#[repr(C)]
#[derive(Debug)]
pub struct QualitySettings {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+QualitySettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::QualitySettings => "UnityEngine"
    ."QualitySettings"
);
#[cfg(feature = "UnityEngine+QualitySettings")]
impl std::ops::Deref for crate::UnityEngine::QualitySettings {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+QualitySettings")]
impl std::ops::DerefMut for crate::UnityEngine::QualitySettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+QualitySettings")]
impl crate::UnityEngine::QualitySettings {
    pub fn OnActiveQualityLevelChanged(
        previousQualityLevel: i32,
        currentQualityLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OnActiveQualityLevelChanged",
                (previousQualityLevel, currentQualityLevel),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_INTERNAL_renderPipeline() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ScriptableObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_INTERNAL_renderPipeline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeColorSpace() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ColorSpace,
    > {
        let __cordl_ret: crate::UnityEngine::ColorSpace = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_activeColorSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_antiAliasing() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_antiAliasing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_desiredColorSpace() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ColorSpace,
    > {
        let __cordl_ret: crate::UnityEngine::ColorSpace = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_desiredColorSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderPipeline() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RenderPipelineAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderPipelineAsset,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_renderPipeline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_INTERNAL_renderPipeline(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_INTERNAL_renderPipeline", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_antiAliasing(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_antiAliasing", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxQueuedFrames(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_maxQueuedFrames", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_renderPipeline(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderPipelineAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_renderPipeline", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vSyncCount(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_vSyncCount", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+QualitySettings")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::QualitySettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
