#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapLightWithIdsGroupEntry_GroupLightData {
    __cordl_parent: crate::System::Object,
    pub _lightGroup: *mut LightGroupSO,
    pub _groupIntensity: f32,
    pub _groupProbeHighlightsIntensityMultiplier: f32,
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData => ""
    ."LightmapLightWithIdsGroupEntry/GroupLightData"
);
#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
impl std::ops::Deref
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
impl crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData {
    pub fn get_groupProbeHighlightsIntensityMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_groupProbeHighlightsIntensityMultiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_groupIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_groupIntensity", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        lightGroup: *mut LightGroupSO,
        groupIntensity: f32,
        groupProbeHighlightsIntensityMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (lightGroup, groupIntensity, groupProbeHighlightsIntensityMultiplier),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_lightGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut LightGroupSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut LightGroupSO = __cordl_object
            .invoke("get_lightGroup", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        lightGroup: *mut LightGroupSO,
        groupIntensity: f32,
        groupProbeHighlightsIntensityMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (lightGroup, groupIntensity, groupProbeHighlightsIntensityMultiplier),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapLightWithIdsGroupEntry_LightIntensitiesWithId {
    __cordl_parent: crate::System::Object,
    pub _lightId: i32,
    pub _intensity: f32,
    pub _probeHighlightsIntensityMultiplier: f32,
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId => ""
    ."LightmapLightWithIdsGroupEntry/LightIntensitiesWithId"
);
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl std::ops::Deref
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId {
    pub fn get_lightId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lightId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_probeHighlightsIntensityMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_probeHighlightsIntensityMultiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_intensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_intensity", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        lightId: i32,
        intensity: f32,
        probeHighlightsIntensityMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightId, intensity, probeHighlightsIntensityMultiplier))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        lightId: i32,
        intensity: f32,
        probeHighlightsIntensityMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (lightId, intensity, probeHighlightsIntensityMultiplier),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapLightWithIdsGroupEntry {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lightmapLightWithIds: *mut LightmapLightWithIds,
    pub _groupLightData: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData,
    >,
    pub _individualLightData: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId,
    >,
    pub _excludedLightIds: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _groupLightsWeighting: ColorMixAndWeightingApproach,
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightmapLightWithIdsGroupEntry => ""
    ."LightmapLightWithIdsGroupEntry"
);
#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
impl std::ops::Deref for LightmapLightWithIdsGroupEntry {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
impl std::ops::DerefMut for LightmapLightWithIdsGroupEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
impl LightmapLightWithIdsGroupEntry {
    #[cfg(feature = "LightmapLightWithIdsGroupEntry+LightIntensitiesWithId")]
    pub type LightIntensitiesWithId = crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId;
    #[cfg(feature = "LightmapLightWithIdsGroupEntry+GroupLightData")]
    pub type GroupLightData = crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData;
    pub fn get_lightmapLightWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut LightmapLightWithIds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut LightmapLightWithIds = __cordl_object
            .invoke("get_lightmapLightWithIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_groupLightsWeighting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<ColorMixAndWeightingApproach> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ColorMixAndWeightingApproach = __cordl_object
            .invoke("get_groupLightsWeighting", ())?;
        Ok(__cordl_ret)
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
    pub fn get_groupLightData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_GroupLightData,
        > = __cordl_object.invoke("get_groupLightData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_individualLightData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LightmapLightWithIdsGroupEntry_LightIntensitiesWithId,
        > = __cordl_object.invoke("get_individualLightData", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LightmapLightWithIdsGroupEntry")]
impl quest_hook::libil2cpp::ObjectType for LightmapLightWithIdsGroupEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
