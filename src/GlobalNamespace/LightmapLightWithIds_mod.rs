#[cfg(feature = "LightmapLightWithIds")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapLightWithIds {
    __cordl_parent: crate::GlobalNamespace::LightWithIds,
    pub _bakeId: crate::GlobalNamespace::LightConstants_BakeId,
    pub _intensity: f32,
    pub _probeIntensity: f32,
    pub _lightIntensityData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LightmapLightWithIds_LightIntensitiesWithId,
            >,
        >,
    >,
    pub _mixType: crate::GlobalNamespace::ColorMixAndWeightingApproach,
    pub _normalizerWeight: f32,
    pub _bakedLightsNormalizer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BakedLightsNormalizer,
    >,
    pub _lightmapLightIdColorPropertyId: i32,
    pub _lightProbeLightIdColorPropertyId: i32,
    pub _initializedPropertyIds: bool,
    pub _initializedNormalizer: bool,
    pub _isNormalizerInScene: bool,
    pub _calculatedColorPreNormalization: crate::UnityEngine::Color,
}
#[cfg(feature = "LightmapLightWithIds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightmapLightWithIds => ""
    ."LightmapLightWithIds"
);
#[cfg(feature = "LightmapLightWithIds")]
impl std::ops::Deref for crate::GlobalNamespace::LightmapLightWithIds {
    type Target = crate::GlobalNamespace::LightWithIds;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIds")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightmapLightWithIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIds")]
impl crate::GlobalNamespace::LightmapLightWithIds {
    #[cfg(feature = "LightmapLightWithIds+LightIntensitiesWithId")]
    pub type LightIntensitiesWithId = crate::GlobalNamespace::LightmapLightWithIds_LightIntensitiesWithId;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBakedLightsNormalizer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBakedLightsNormalizer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLightWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightWithIds_LightWithId,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightWithIds_LightWithId,
                >,
            >,
        > = __cordl_object.invoke("GetLightWithIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessNewColorData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNewColorData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetChannelColorDirect(
        &mut self,
        channelColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetChannelColorDirect", (channelColor))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDataToShaders(
        &mut self,
        lightmapColor: crate::UnityEngine::Color,
        probeColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDataToShaders", (lightmapColor, probeColor))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetShaderProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetShaderProperties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bakeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::LightConstants_BakeId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::LightConstants_BakeId = __cordl_object
            .invoke("get_bakeId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_calculatedColorPreNormalization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_calculatedColorPreNormalization", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_intensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_intensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mixType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ColorMixAndWeightingApproach,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ColorMixAndWeightingApproach = __cordl_object
            .invoke("get_mixType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalizerWeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_normalizerWeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_intensity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_intensity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_normalizerWeight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_normalizerWeight", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightmapLightWithIds")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LightmapLightWithIds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightmapLightWithIds+LightIntensitiesWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapLightWithIds_LightIntensitiesWithId {
    __cordl_parent: crate::GlobalNamespace::LightWithIds_LightWithId,
    pub _intensity: f32,
    pub _probeHighlightsIntensityMultiplier: f32,
}
#[cfg(feature = "LightmapLightWithIds+LightIntensitiesWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LightmapLightWithIds_LightIntensitiesWithId => ""
    ."LightmapLightWithIds/LightIntensitiesWithId"
);
#[cfg(feature = "LightmapLightWithIds+LightIntensitiesWithId")]
impl std::ops::Deref
for crate::GlobalNamespace::LightmapLightWithIds_LightIntensitiesWithId {
    type Target = crate::GlobalNamespace::LightWithIds_LightWithId;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIds+LightIntensitiesWithId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LightmapLightWithIds_LightIntensitiesWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIds+LightIntensitiesWithId")]
impl crate::GlobalNamespace::LightmapLightWithIds_LightIntensitiesWithId {
    pub fn New(
        lightId: i32,
        lightIntensity: f32,
        probeMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightId, lightIntensity, probeMultiplier))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lightId: i32,
        lightIntensity: f32,
        probeMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightId, lightIntensity, probeMultiplier))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_intensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_intensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_probeHighlightsIntensityMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_probeHighlightsIntensityMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightmapLightWithIds+LightIntensitiesWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightmapLightWithIds_LightIntensitiesWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
