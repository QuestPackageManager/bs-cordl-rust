#[cfg(
    feature = "UnityEngine+Rendering+SupportedRenderingFeatures+LightmapMixedBakeModes"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedRenderingFeatures_LightmapMixedBakeModes {
    IndirectOnly = 1i32,
    None = 0i32,
    Shadowmask = 4i32,
    Subtractive = 2i32,
}
#[cfg(
    feature = "UnityEngine+Rendering+SupportedRenderingFeatures+LightmapMixedBakeModes"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes =>
    "UnityEngine.Rendering"."SupportedRenderingFeatures/LightmapMixedBakeModes"
);
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures+ReflectionProbeModes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedRenderingFeatures_ReflectionProbeModes {
    None = 0i32,
    Rotation = 1i32,
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures+ReflectionProbeModes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::SupportedRenderingFeatures_ReflectionProbeModes =>
    "UnityEngine.Rendering"."SupportedRenderingFeatures/ReflectionProbeModes"
);
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
#[repr(C)]
#[derive(Debug)]
pub struct SupportedRenderingFeatures {
    __cordl_parent: crate::System::Object,
    pub _reflectionProbeModes_k__BackingField: crate::UnityEngine::Rendering::SupportedRenderingFeatures_ReflectionProbeModes,
    pub _defaultMixedLightingModes_k__BackingField: crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
    pub _mixedLightingModes_k__BackingField: crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
    pub _lightmapBakeTypes_k__BackingField: crate::UnityEngine::LightmapBakeType,
    pub _lightmapsModes_k__BackingField: crate::UnityEngine::LightmapsMode,
    pub _enlightenLightmapper_k__BackingField: bool,
    pub _enlighten_k__BackingField: bool,
    pub _lightProbeProxyVolumes_k__BackingField: bool,
    pub _motionVectors_k__BackingField: bool,
    pub _receiveShadows_k__BackingField: bool,
    pub _reflectionProbes_k__BackingField: bool,
    pub _reflectionProbesBlendDistance_k__BackingField: bool,
    pub _rendererPriority_k__BackingField: bool,
    pub _rendersUIOverlay_k__BackingField: bool,
    pub _overridesEnvironmentLighting_k__BackingField: bool,
    pub _overridesFog_k__BackingField: bool,
    pub _overridesRealtimeReflectionProbes_k__BackingField: bool,
    pub _overridesOtherLightingSettings_k__BackingField: bool,
    pub _editableMaterialRenderQueue_k__BackingField: bool,
    pub _overridesLODBias_k__BackingField: bool,
    pub _overridesMaximumLODLevel_k__BackingField: bool,
    pub _overridesEnableLODCrossFade_k__BackingField: bool,
    pub _rendererProbes_k__BackingField: bool,
    pub _particleSystemInstancing_k__BackingField: bool,
    pub _autoAmbientProbeBaking_k__BackingField: bool,
    pub _autoDefaultReflectionProbeBaking_k__BackingField: bool,
    pub _overridesShadowmask_k__BackingField: bool,
    pub _overridesLightProbeSystem_k__BackingField: bool,
    pub _supportsHDR_k__BackingField: bool,
    pub _overridesLightProbeSystemWarningMessage_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::SupportedRenderingFeatures => "UnityEngine.Rendering"
    ."SupportedRenderingFeatures"
);
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
impl std::ops::Deref for crate::UnityEngine::Rendering::SupportedRenderingFeatures {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::SupportedRenderingFeatures {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
impl crate::UnityEngine::Rendering::SupportedRenderingFeatures {
    #[cfg(
        feature = "UnityEngine+Rendering+SupportedRenderingFeatures+LightmapMixedBakeModes"
    )]
    pub type LightmapMixedBakeModes = crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes;
    #[cfg(
        feature = "UnityEngine+Rendering+SupportedRenderingFeatures+ReflectionProbeModes"
    )]
    pub type ReflectionProbeModes = crate::UnityEngine::Rendering::SupportedRenderingFeatures_ReflectionProbeModes;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_autoAmbientProbeBaking(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_autoAmbientProbeBaking", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_autoDefaultReflectionProbeBaking(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_autoDefaultReflectionProbeBaking", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultMixedLightingModes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes = __cordl_object
            .invoke("get_defaultMixedLightingModes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enlighten(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enlighten", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enlightenLightmapper(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enlightenLightmapper", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lightmapBakeTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LightmapBakeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LightmapBakeType = __cordl_object
            .invoke("get_lightmapBakeTypes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lightmapsModes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LightmapsMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LightmapsMode = __cordl_object
            .invoke("get_lightmapsModes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mixedLightingModes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes = __cordl_object
            .invoke("get_mixedLightingModes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_overridesLightProbeSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_overridesLightProbeSystem", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rendersUIOverlay(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_rendersUIOverlay", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::SupportedRenderingFeatures {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
